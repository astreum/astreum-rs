use std::{sync::{Mutex, Arc}, error::Error};

use crate::relay::Relay;

use self::object::Object;
mod index;
mod list;
mod search;
pub mod object;
mod miner;
mod pairs;
pub struct Storage {
    pub object_store_pointer: Arc<Mutex<neutrondb::Store<[u8;32], Object>>>,
    pub relay_pointer: Arc<Mutex<Relay>>
}

impl Storage {

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

    pub fn local_get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
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

    pub fn get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
        match self.local_get_object(object_hash) {
            Ok(object) => Ok(object),
            Err(_) => match self.relay_pointer.lock() {
                Ok(relay) => {
                    relay.network_get_object(object_hash)
                },
                Err(_) => Err("not found!")?,
            }
        }
    }

	pub fn object_children(&self, root_hash: &[u8;32]) -> Result<Vec<Object>, Box<dyn Error>> {

        let root_object = self.get_object(root_hash)?;

        if root_object.leaf {
            return Err("Tree Leaf!")?;
        }

        match root_object.data.len() {
            32 => {
                let child_object = self.get_object(root_object.data[..].try_into()?)?;
                Ok(vec![child_object])
            },
            64 => {
                let left_object = self.get_object(root_object.data[..32].try_into()?)?;
                let right_object = self.get_object(root_object.data[32..].try_into()?)?;
                Ok(vec![left_object, right_object])
            },
            _ => Err("Children Hash format error!")?
        }
    }

}