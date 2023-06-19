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

    pub fn local_get(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
        match self.object_store_pointer.lock() {
            Ok(object_store) => {
                object_store.get(&object_hash)
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

    pub fn local_put(&self, object: Object) -> Result<(), Box<dyn Error>> {
        match self.object_store_pointer.lock() {
            Ok(mut object_store) => {
                object_store.put(&object.hash(), &object)
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

    pub fn global_get(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
        match self.local_get(object_hash) {
            Ok(object) => Ok(object),
            Err(_) => self.network_get(object_hash)
        }
    }

    pub fn send_message(&self, ip_addr: IpAddr, message: Message) -> Result<(), Box<dyn Error>> {
        match self.outgoing_queue_pointer.lock() {
            Ok(mut outgoing_queue) => {
                outgoing_queue.push((ip_addr, message));
                Ok(())
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

    pub fn network_get(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {

        todo!()

        // let now = Instant::now();

        // let nearest_peer = self.nearest_peer(object_hash)?;

        // let mut result = Err("Not Found!")?;

        // let object_request = ObjectRequest {
        //     hash: *object_hash,
        //     kind: ObjectRequestKind::Get
        // };
        
        // let object_request_message = Message {
        //     body: object_request.into(),
        //     topic: Topic::ObjectRequest
        // };

        // let _c = self.send_message(nearest_peer, object_request_message)?;

        // for _ in 0..3 {
        //     for _ in 0..10 {
        //         sleep(time::Duration::from_millis(100));
        //         match self.local_get(object_hash) {
        //             Ok(object) => {
        //                 result = Ok(object);
        //                 break;
        //             },
        //             Err(_) => (),
        //         }
        //     }
        //     match result {
        //         Ok(_) => break,
        //         Err(_) => {
        //             let random_peer = self.nearest_peer(object_hash)?;
        //             let _c = self.send_message(random_peer, object_request_message.clone())?;
        //         },
        //     }
        // }

        // result

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

    pub fn storage_list(&self, root_hash: &[u8; 32]) -> Result<Vec<Object>, Box<dyn Error>> {

        let root_object = self.global_get(root_hash)?;

        // Put root hash object into a vec
        let mut object_vec = vec![self.local_get(root_hash)?];

        // Map the vec objects into the children until the first object in the vec is a leaf
        while !object_vec[0].leaf {
            let new_object_vec: Vec<_> = object_vec
                .iter()
                .flat_map(|x| {
                    if x.leaf {
                        vec![x.clone()]
                    } else {
                        let child_hashes = split_hash(&x.data);

                        // Retrieve child objects from the store and collect them into a new vec
                        let child_objects: Result<Vec<_>, _> = child_hashes
                            .iter()
                            .map(|hash| self.global_get(hash))
                            .collect();

                        child_objects.and_then(|objects| Ok(objects)).unwrap_or_default()
                        
                    }
                })
                .collect();

            object_vec = new_object_vec;
        }

        Ok(object_vec)
    }


    pub fn first_key_value(&self, root_hash: &[u8; 32]) -> Result<(Object, Object), Box<dyn Error>> {

        let mut children = self.object_children(root_hash)?;

        while !children[0].leaf {
            children = self.object_children(&children[0].hash())?
        }

        if children.len() == 2 {
            Ok((children[0], children[1]))
        } else {
            Err("Formatting error!")?
        }

    }

    pub fn middle_key_value(&self, root_hash: &[u8; 32]) -> Result<(Object, Object), Box<dyn Error>> {

        let mut children = self.object_children(root_hash)?;

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

    pub fn object_children(&self, parent_hash: &[u8;32]) -> Result<Vec<Object>, Box<dyn Error>> {

        let root_object = self.global_get(parent_hash)?;

        if root_object.leaf {
            return Err("Tree Leaf!")?;
        }

        match root_object.data.len() {
            32 => {
                let child_object = self.global_get(root_object.data[..].try_into()?)?;
                Ok(vec![child_object])
            },
            64 => {
                let left_object = self.global_get(root_object.data[..32].try_into()?)?;
                let right_object = self.global_get(root_object.data[32..].try_into()?)?;
                Ok(vec![left_object, right_object])
            },
            _ => Err("Children Hash format error!")?
        }
    }

    pub fn storage_search(
        &self,
        key: &[u8],
        root_hash: &[u8; 32],
    ) -> Result<Object, Box<dyn Error>> {

        let mut top_node = self.global_get(root_hash)?;

        while !top_node.leaf {

            let middle_key_value = self.middle_key_value(&top_node.hash())?;

            if middle_key_value.0.data == key {
                return Ok(middle_key_value.1);
            } else {
                let children = self.object_children(&top_node.hash())?;
                if children.len() == 2 {
                    if key < &middle_key_value.0.data {
                        top_node = children[0].clone()
                    } else {
                        top_node = children[1].clone()
                    }
                } else {
                    Err("Formatting error!")?
                }
            }
        }

        Err("Not found!")?

    }    

}
