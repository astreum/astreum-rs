use std::error::Error;

use super::object::Object;

pub struct LocalGetResult {
    object: Option<Object>,
    missing: Vec<[u8;32]>
}

pub fn storage_index(
    index: usize,
    object_store: &neutrondb::Store<[u8; 32], Object>,
    root_hash: [u8; 32],
) -> Result<LocalGetResult, Box<dyn Error>> {
    let mut current_level = vec![root_hash];
    let mut next_capacity = 2;
    let mut first_leaf_index = 0;
    let mut level = 0;
    let mut missing_hashes = Vec::new();

    while !current_level.is_empty() {
        if next_capacity <= index {
            let mut new_current_level = Vec::new();
            for hash in current_level {
                if let object = object_store.get(&hash)? {
                    let child_hashes = split_hash(&object.data);
                    new_current_level.extend(child_hashes);
                } else {
                    missing_hashes.push(hash);
                }
            }
            current_level = new_current_level;
        } else {
            if let Some(first_object_hash) = current_level.get(0) {
                if let first_object = object_store.get(first_object_hash)? {
                    if first_object.leaf {
                        let leaf_index = index - first_leaf_index;
                        if let Some(leaf_hash) = current_level.get(leaf_index) {
                            if let object = object_store.get(leaf_hash)? {
                                return Ok(LocalGetResult {
                                    object: Some(object.clone()),
                                    missing: missing_hashes,
                                });
                            }
                        }
                    } else {
                        let right_half_capacity = current_level.len() / 2;
                        if index >= right_half_capacity {
                            let new_current_level = current_level[right_half_capacity..]
                                .iter()
                                .map(|hash| *hash)
                                .collect::<Vec<_>>();
                            current_level = new_current_level;
                            first_leaf_index += right_half_capacity;
                        } else {
                            let new_current_level = current_level[0..right_half_capacity]
                                .iter()
                                .map(|hash| *hash)
                                .collect::<Vec<_>>();
                            current_level = new_current_level;
                        }
                    }
                }
            }
        }

        level += 1;
        next_capacity = 2_usize.pow(level);
    }

    Ok(LocalGetResult {
        object: None,
        missing: missing_hashes,
    })
}

fn split_hash(data: &[u8]) -> Vec<[u8; 32]> {
    // Split the data into fixed-size chunks (32 bytes)
    data.chunks_exact(32)
        .map(|chunk| {
            let mut hash = [0; 32];
            hash.copy_from_slice(chunk);
            hash
        })
        .collect()
}

pub fn storage_search(
    key: &[u8],
    object_store: &neutrondb::Store<[u8; 32], Object>,
    root_hash: [u8; 32],
) -> Result<LocalGetResult, Box<dyn Error>> {
    let mut current_level = vec![root_hash];
    let mut parent_hashes = Vec::new();
    let mut missing_hashes = Vec::new();

    while !current_level.is_empty() {
        let middle_index = current_level.len() / 2;
        let middle_hash = current_level[middle_index];
        if let middle_object = object_store.get(&middle_hash)? {
            if middle_object.leaf && middle_object.data == key {
                // Found the matching leaf, return the second child
                let child_hashes = split_hash(&middle_object.data);
                if let second_child = object_store.get(&child_hashes[1])? {
                    return Ok(LocalGetResult {
                        object: Some(second_child.clone()),
                        missing: missing_hashes,
                    });
                }
            }

            if key < &middle_object.data || !middle_object.leaf {
                // Go left
                let left_hashes = current_level[..middle_index].to_vec();
                let right_hashes = current_level[middle_index + 1..].to_vec();
                current_level = left_hashes;
                parent_hashes.extend(right_hashes);
            } else {
                // Go right
                let right_hashes = current_level[middle_index + 1..].to_vec();
                current_level = right_hashes;
                parent_hashes.extend(current_level[..middle_index].to_vec());
            }
        } else {
            // Object is missing, add its hash to missing_hashes
            missing_hashes.push(middle_hash);
            // Remove the missing hash from current_level
            current_level.remove(middle_index);
        }
    }

    // Trace back to the top while branching left or right
    while let Some(parent_hash) = parent_hashes.pop() {
        if let parent_object = object_store.get(&parent_hash)? {
            if key < &parent_object.data {
                // Go left
                let right_hashes = current_level;
                current_level = parent_hashes;
                parent_hashes = right_hashes;
            } else {
                // Go right
                let left_hashes = current_level;
                current_level = parent_hashes;
                parent_hashes = left_hashes;
            }

            if let Some(middle_hash) = current_level.pop() {
                if let middle_object = object_store.get(&middle_hash)? {
                    if middle_object.leaf && middle_object.data == key {
                        // Found the matching leaf, return the second child
                        let child_hashes = split_hash(&middle_object.data);
                        if let second_child = object_store.get(&child_hashes[1])? {
                            return Ok(LocalGetResult {
                                object: Some(second_child.clone()),
                                missing: missing_hashes,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(LocalGetResult {
        object: None,
        missing: missing_hashes,
    })
}

pub struct LocalListResult {
    objects: Vec<Object>,
    missing: Vec<[u8; 32]>,
}

pub fn storage_list(
    object_store: &neutrondb::Store<[u8; 32], Object>,
    root_hash: [u8; 32],
) -> Result<LocalListResult, Box<dyn Error>> {
    // Put root hash object into a vec
    let mut object_vec = vec![object_store.get(&root_hash)?];
    let mut missing_hashes = Vec::new();

    // Map the vec objects into the children until the first object in the vec is a leaf
    while !object_vec[0].leaf {
        let new_object_vec: Vec<_> = object_vec
            .iter()
            .flat_map(|x| {
                if x.leaf {
                    vec![x.clone()]
                } else {
                    let child_hashes = split_hash(&x.data);

                    // Retrieve child objects from the store and separate the missing objects
                    let mut found_objects = Vec::new();

                    for hash in child_hashes {

                        match object_store.get(&hash) {

                            Ok(object) => found_objects.push(object),

                            Err(_) => missing_hashes.push(hash),

                        }

                    }

                    found_objects
                }
            })
            .collect();

        object_vec = new_object_vec;
    }

    Ok(LocalListResult {
        objects: object_vec,
        missing: missing_hashes,
    })
}
