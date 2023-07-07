use std::{error::Error, result};

use super::{Storage, object::Object};

impl Storage {
    pub fn get_index(&self, index: usize, root_hash: &[u8; 32]) -> Result<Object, Box<dyn Error>> {
        let mut current_level = vec![root_hash.clone()];
        let mut next_capacity = 2;
        let mut first_leaf_index = 0;
        let mut level = 0;

        while !current_level.is_empty() {
            if next_capacity <= index {
                let mut new_current_level = Vec::new();
                for object_hash in current_level {
                    let object = self.get_object(&object_hash)?;
                    let child_hashes = object.branch_hashes();
                    new_current_level.extend(child_hashes);
                }
                current_level = new_current_level;
            } else {
                if let Some(first_object_hash) = current_level.get(0) {
                    let first_object = self.get_object(first_object_hash)?;
                    if first_object.leaf {
                        let leaf_index = index - first_leaf_index;
                        match current_level.get(leaf_index) {
                            Some(object_hash) => return self.get_object(object_hash),
                            None => Err("Internal Error!")?,
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

            level += 1;
            next_capacity = 2_usize.pow(level);
        }
        Err("Not found!")?
    }
}
