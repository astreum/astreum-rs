use std::{error::Error, sync::{Arc, Mutex}};

use super::{chain::ChainID, address::Address, object::Object};

#[derive(Clone, Debug)]
pub struct Transaction {
	pub chain_id: ChainID,
	pub counter: opis::Integer,
	pub data: Vec<u8>,
	pub details_hash: [u8;32],
	pub recipient: Address,
	pub sender: Address,
	pub signature: [u8;64],
	pub transaction_hash: [u8;32],
	pub value: opis::Integer,
}

impl Transaction {

	pub fn sign(&mut self, secret_key: &[u8; 32]) -> Result<(), Box<dyn Error>> {

		let signature = fides::ed25519::sign(&self.details_hash, secret_key)?;

		self.signature = signature;

		Ok(())

	}

	pub fn update_details_hash(&mut self) {

		self.details_hash = self.details_hash()
		
	}

	pub fn details_hash(&self) -> [u8; 32] {

		let chain_bytes: Vec<u8> = (&self.chain_id).into();

		let counter_bytes: Vec<u8> = (&self.counter).into();

		let value_bytes: Vec<u8> = (&self.value).into();

		fides::merkle_tree::root(
			fides::hash::blake_3,
			&[
				&chain_bytes,
				&counter_bytes,
				&self.data,
				&self.recipient.0,
				&self.sender.0,
				&value_bytes
			]
		)

	}

	// pub fn from_astreum_storage(
	// 	transaction_hash: &[u8;32],
	// 	astreum_storage_pointer: Arc<Mutex<neutrondb::Store<[u8;32], Object>>>,
	// 	relay: &Relay
	// ) -> Result<Transaction, Box<dyn Error>> {

	// 	let check = match astreum_storage_pointer.lock() {
	// 		Ok(astreum_storage) => {
	// 			match astreum_storage.get(key) {
	// 				Ok(object) => Some(object),
	// 				Err(_) => None,
	// 			}
	// 		},
	// 		Err(_) => Err("astreum storage lock error!")?,
	// 	};

	// 	match check {

	// 		Some(object) => {
	// 			result = Ok(object);
	// 			break;
	// 		},

	// 		None => {

	// 			// send object request to relay

	// 			// sleep for 1 sec

	// 		}

	// 	}

	// }

}

impl TryFrom<&[u8]> for Transaction {

   type Error = Box<dyn Error>;

   fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

	   let tx_decoded = astro_format::decode(value)?;

	   if tx_decoded.len() == 6 {

		   let tx = Transaction {
			   chain_id: tx_decoded[0].try_into()?,
			   counter: tx_decoded[1].try_into()?,
			   data: tx_decoded[2].try_into()?,
			   recipient: tx_decoded[3].try_into()?,
			   sender: tx_decoded[4].try_into()?,
			   signature: tx_decoded[5].try_into()?,
			   transaction_hash: tx_decoded[6].try_into()?,
			   value: tx_decoded[7].try_into()?,
			   details_hash: [0; 32],
		   };

		   Ok(tx)

	   } else {

		   Err("Internal error!")?

	   }

   }

}

impl Into<Vec<u8>> for Transaction {
   fn into(self) -> Vec<u8> {
	   (&self).into()
   }
}

impl Into<Vec<u8>> for &Transaction {

   fn into(self) -> Vec<u8> {

	   let chain_bytes: Vec<u8> = (&self.chain_id).into();

	   let counter_bytes: Vec<u8> = (&self.counter).into();

	   let value_bytes: Vec<u8> = (&self.value).into();

	   astro_format::encode(&[
		   &chain_bytes,
		   &counter_bytes,
		   &self.data,
		   &self.recipient.0,
		   &self.sender.0,
		   &value_bytes,
	   ])

   }

}