use std::{error::Error, time::{Instant, self}, net::IpAddr, thread::sleep, result};

use super::{object::{Object, ObjectRequest, ObjectRequestKind}, App, message::Message, topic::Topic};

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


impl App {

    pub fn send_message(&self, ip_addr: IpAddr, message: Message) -> Result<(), Box<dyn Error>> {
        match self.outgoing_queue_pointer.lock() {
            Ok(mut outgoing_queue) => {
                outgoing_queue.push((ip_addr, message));
                Ok(())
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

    pub fn nearest_peer(&self, hash: &[u8;32]) -> Result<IpAddr, Box<dyn Error>> {

        todo!()

    }

    pub fn random_peer(&self, hash: &[u8;32]) -> Result<IpAddr, Box<dyn Error>> {

        todo!()

    }

    pub fn network_put(&self, object: Object) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn first_key_value(&self, root_hash: &[u8; 32]) -> Result<(Object, Object), Box<dyn Error>> {

        let mut children = self.object_children(root_hash)?;

        while !children[0].leaf {
            children = self.object_children(&children[0].hash())?
        }

        if children.len() == 2 {
            Ok((children[0].clone(), children[1].clone()))
        } else {
            Err("Formatting error!")?
        }

    }

    pub fn middle_key_value(&self, root_hash: &[u8; 32]) -> Result<(Object, Object), Box<dyn Error>> {

        let children = self.object_children(root_hash)?;

        if children.len() == 2 {
            if !children[0].leaf {
                self.first_key_value(&children[1].hash())
            } else {
                Ok((children[0].clone(), children[1].clone()))
            }
        } else {
            Err("Formatting error!")?
        }
        
    }    

}
