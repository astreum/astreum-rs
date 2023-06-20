use std::{error::Error, time::{Instant, self}, thread::sleep};

use super::{App, message::Message, topic::Topic};
#[derive(PartialEq)]
#[derive(Clone, Debug)]
pub struct Object {
	pub leaf: bool,
	pub data: Vec<u8>
}
pub struct ObjectRequest {
	hash: [u8;32],
	kind: ObjectRequestKind,
 }

impl Into<Vec<u8>> for ObjectRequest {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
 
pub enum ObjectRequestKind {
	Get,
	Put
}

impl Into<Vec<u8>> for ObjectRequestKind {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
pub struct ObjectResponse{
	data: Vec<u8>,
	kind: ObjectResponseKind,
	hash: [u8;32],
	request: ObjectRequestKind
 }
 
 pub enum ObjectResponseKind { Ok, Next }

impl Object {

	pub fn hash(&self) -> [u8;32] {
		todo!()
	}

	pub fn branch_hashes(&self) -> Vec<[u8; 32]> {
		self.data
			.chunks_exact(32)
			.map(|chunk| {
				let mut hash = [0; 32];
				hash.copy_from_slice(chunk);
				hash
			})
			.collect()
	}

}


impl Into<Vec<u8>> for Object {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

impl TryFrom<Vec<u8>> for Object {

	type Error = Box<dyn Error>;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		  Object::try_from(&value[..])
	 }

}

impl TryFrom<&[u8]> for Object {

	type Error = Box<dyn Error>;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {

		let object_fields = astro_format::decode(value)?;
		
		if object_fields.len() == 2 {

			let object = Object {
				leaf: match object_fields[0] {
					[0] => false,
					[1] => true,
					_ => Err("object type error!")?
				},
				data: object_fields[1].to_vec(),
			};

			Ok(object)

		} else {
			Err("object fields error!")?
		}

	}

}

impl App {

    pub fn local_get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
        match self.object_store_pointer.lock() {
            Ok(object_store) => {
                object_store.get(&object_hash)
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

	pub fn network_get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
		
        let now = Instant::now();

        let nearest_peer = self.nearest_peer(object_hash)?;

        let mut result = Err("Not Found!")?;

        let object_request = ObjectRequest {
            hash: *object_hash,
            kind: ObjectRequestKind::Get
        };
        
        let object_request_message = Message {
            body: object_request.into(),
            topic: Topic::ObjectRequest
        };

        let _c = self.send_message(nearest_peer, object_request_message)?;

        for _ in 0..3 {
            for _ in 0..10 {
                sleep(time::Duration::from_millis(100));
                match self.local_get_object(object_hash) {
                    Ok(object) => {
                        result = Ok(object);
                        break;
                    },
                    Err(_) => (),
                }
            }
            match result {
                Ok(_) => break,
                Err(_) => {
                    let random_peer = self.nearest_peer(object_hash)?;
                    let _c = self.send_message(random_peer, object_request_message.clone())?;
                },
            }
        }

        result

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
            Err(_) => self.network_get_object(object_hash)
        }
    }

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