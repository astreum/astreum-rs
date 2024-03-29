use std::error::Error;

use crate::storage::Storage;

#[derive(Clone, Debug)]
pub struct Receipt {
	pub hash: [u8;32],
	pub solar_used: u64,
	pub status: Status,
}

impl Receipt {

pub fn hash(&mut self) {

	let solar_used_bytes: Vec<u8> = opis::Integer::from(&self.solar_used).into();
	let status_bytes: Vec<u8> = (&self.status).into();

	self.hash = fides::merkle_tree::root(
		fides::hash::blake_3,
		&[&solar_used_bytes, &status_bytes]
	)

}

pub fn new(solar_used: u64, status: Status) -> Self {
	let mut receipt = Receipt {
		solar_used,
		status,
		hash: [0_u8;32],
	};
	receipt.hash();
	receipt
}

}

#[derive(Clone, Debug)]
pub enum Status {
Accepted,
BalanceError,
SolarError
}

impl Into<Vec<u8>> for Status {
fn into(self) -> Vec<u8> {
	(&self).into()
}
}

impl Into<Vec<u8>> for &Status {

fn into(self) -> Vec<u8> {

	match self {
		Status::Accepted => vec![1_u8],
		Status::BalanceError => vec![2_u8],
		Status::SolarError => vec![3_u8]
	}

}

}

impl TryFrom<&[u8]> for Status {
	type Error = Box<dyn Error>;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		match value {
			[1_u8] => Ok(Status::Accepted),
			[2_u8] => Ok(Status::BalanceError),
			[3_u8] => Ok(Status::SolarError),
			_ => Err("error!")?
			
		}
	}
}

impl Storage {

pub fn get_receipt(&self, receipt_hash: &[u8;32]) -> Result<Receipt, Box<dyn Error>> {

	let receipt_objects = self.object_children(receipt_hash)?;

	let receipt = Receipt {
		solar_used: u64::from_be_bytes(receipt_objects[0].data[..].try_into()?),
		status: Status::try_from(&receipt_objects[1].data[..])?,
		hash: [0_u8;32],
	};

	Ok(receipt)

}

}
