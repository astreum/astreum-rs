use std::{error::Error, sync::{Arc, Mutex}};

#[derive(Clone, Debug)]
pub struct Object {
	pub leaf: bool,
	pub data: Vec<u8>
}


pub struct ObjectRequest {
	hash: [u8;32],
	kind: ObjectRequestKind,
 }
 
 pub enum ObjectRequestKind { Get, Put }

pub struct ObjectResponse{
	data: Vec<u8>,
	kind: ObjectResponseKind,
	hash: [u8;32],
	request: ObjectRequestKind
 }
 
 pub enum ObjectResponseKind { Ok, Next }

impl Object {

	pub fn from_astreum_storage(
		key: &[u8;32],
		astreum_storage_pointer: Arc<Mutex<neutrondb::Store<[u8;32], Object>>>,
	) -> Result<Object, Box<dyn Error>> {

		let mut result: Result<Object, Box<dyn Error>> = Err("object not found!")?;
		
		for _ in 0..10 {

			let check = match astreum_storage_pointer.lock() {
				Ok(astreum_storage) => {
					match astreum_storage.get(key) {
						Ok(object) => Some(object),
						Err(_) => None,
					}
				},
				Err(_) => Err("astreum storage lock error!")?,
			};

			match check {

				Some(object) => {
					result = Ok(object);
					break;
				},

				None => {

					// send object request to relay

					// sleep for 1 sec

				}

			}
			
		}

		result

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
