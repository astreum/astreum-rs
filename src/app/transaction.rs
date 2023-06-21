use std::error::Error;
use super::address::Address;
use super::App;
use super::chain::ChainID;

#[derive(Clone, Debug)]
pub struct Transaction {
	pub chain_id: ChainID,
	pub counter: opis::Integer,
	pub data: Vec<u8>,
	pub recipient: Address,
	pub sender: Address,
	pub signature: [u8;64],
	pub value: opis::Integer,
}

impl Transaction {

	pub fn sign(&mut self, secret_key: &[u8; 32]) -> Result<(), Box<dyn Error>> {
		self.signature = fides::ed25519::sign(&self.details_hash(), secret_key)?;
		Ok(())
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

}

// impl TryFrom<&[u8]> for Transaction {

//    type Error = Box<dyn Error>;

//    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

// 	   let tx_decoded = astro_format::decode(value)?;

// 	   if tx_decoded.len() == 6 {

// 		   let tx = Transaction {
// 			   chain_id: tx_decoded[0].try_into()?,
// 			   counter: tx_decoded[1].try_into()?,
// 			   data: tx_decoded[2].try_into()?,
// 			   recipient: tx_decoded[3].try_into()?,
// 			   sender: tx_decoded[4].try_into()?,
// 			   signature: tx_decoded[5].try_into()?,
// 			   value: tx_decoded[7].try_into()?
// 		   };

// 		   Ok(tx)

// 	   } else {

// 		   Err("Internal error!")?

// 	   }

//    }

// }

// impl Into<Vec<u8>> for Transaction {
//    fn into(self) -> Vec<u8> {
// 	   (&self).into()
//    }
// }

// impl Into<Vec<u8>> for &Transaction {

//    fn into(self) -> Vec<u8> {

// 	   let chain_bytes: Vec<u8> = (&self.chain_id).into();

// 	   let counter_bytes: Vec<u8> = (&self.counter).into();

// 	   let value_bytes: Vec<u8> = (&self.value).into();

// 	   astro_format::encode(&[
// 		   &chain_bytes,
// 		   &counter_bytes,
// 		   &self.data,
// 		   &self.recipient.0,
// 		   &self.sender.0,
// 		   &value_bytes,
// 	   ])

//    }

// }

impl App {

    pub fn get_transaction(&self, transaction_hash: &[u8;32]) -> Result<Transaction, Box<dyn Error>> {

		let transaction_objects = self.object_children(transaction_hash)?;

        let detail_objects = self.get_list(&transaction_objects[0].hash())?;

		let chain_id = ChainID::try_from(&detail_objects[0].data[..])?;

		let transaction = Transaction {
			chain_id,
			counter: detail_objects[1].data[..].into(),
			data: detail_objects[2].data.clone(),
			recipient: Address(detail_objects[3].data[..].try_into()?),
			sender: Address(detail_objects[4].data[..].try_into()?),
			signature: transaction_objects[1].data[..].try_into()?,
			value: detail_objects[5].data[..].into(),
		};

		Ok(transaction)

	}

}
