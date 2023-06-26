use std::error::Error;

#[derive(PartialEq, Clone, Debug)]
pub struct Object {
	pub leaf: bool,
	pub data: Vec<u8>
}

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