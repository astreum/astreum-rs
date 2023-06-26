use std::error::Error;

use super::{Storage, object::Object};

impl Storage {
    pub fn get_list(&self, root_hash: &[u8; 32]) -> Result<Vec<Object>, Box<dyn Error>> {

        let mut list = vec![self.get_object(root_hash)?];

        while !list[0].leaf {

			let mut new_list = vec![];

			for object in list {
				if object.leaf {
					new_list.push(object)
				} else{
					let branch_hashes = object.branch_hashes();
					for branch_hash in branch_hashes {
						new_list.push(self.get_object(&branch_hash)?)
					}
				}
			}

			list = new_list

		}

        Ok(list)

    }
}