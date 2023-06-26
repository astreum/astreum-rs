use std::error::Error;

use super::{Storage, object::Object};

impl Storage {
    pub fn search_object(&self, key: &[u8], root_hash: &[u8; 32]) -> Result<Object, Box<dyn Error>> {

        let mut top_node = self.get_object(root_hash)?;

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