use std::error::Error;

use super::{chain::ChainID, transaction::Transaction, address::Address};

#[derive(Clone, Debug)]
pub struct Block {
   pub accounts_hash: [u8; 32],
   pub block_hash: [u8; 32],
   pub chain_id: ChainID,
   pub data: Vec<u8>,
   pub delay_difficulty: u64,
   pub delay_output: Vec<u8>,
   pub details_hash: [u8; 32],
   pub number: opis::Integer,
   pub previous_block_hash: [u8; 32],
   pub receipts_hash: [u8; 32],
   pub signature: [u8; 64],
   pub solar_used: u64,
   pub time: u64,
   pub transactions: Vec<Transaction>,
   pub transactions_hash: [u8; 32],
   pub validator: Address,
}

impl Block {

   pub fn sign(&mut self, secret_key: &[u8; 32]) -> Result<(), Box<dyn Error>> {

       self.signature = fides::ed25519::sign(&self.details_hash, secret_key)?;

       Ok(())

   }

   pub fn update_details_hash(&mut self) {

      self.details_hash = self.details_hash()
      
  }

  pub fn details_hash(&self) -> [u8; 32] {

      let chain_bytes: Vec<u8> = (&self.chain_id).into();

      let delay_difficulty_bytes: Vec<u8> = opis::Integer::from(&self.delay_difficulty).into();

      let number_bytes: Vec<u8> = (&self.number).into();

      let solar_used_bytes: Vec<u8> = opis::Integer::from(&self.solar_used).into();

      let time_bytes: Vec<u8> = opis::Integer::from(&self.time).into();

      fides::merkle_tree::root(
         fides::hash::blake_3,
          &[
              &self.accounts_hash,
              &chain_bytes,
              &self.data,
              &delay_difficulty_bytes,
              &self.delay_output,
              &number_bytes,
              &self.previous_block_hash,
              &self.receipts_hash,
              &self.signature,
              &solar_used_bytes,
              &time_bytes,
              &self.transactions_hash,
              &self.validator.0,
          ]
      )
  
   }
  
   pub fn update_block_hash(&mut self) {
      
      self.block_hash = self.block_hash()
   
   }

   pub fn block_hash(&self) -> [u8; 32] {

      fides::merkle_tree::root(fides::hash::blake_3, &[&self.details_hash, &self.signature])

   }
   
}

impl TryFrom<&[u8]> for Block {

   type Error = Box<dyn Error>;

   fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
       
       let block_details = astro_format::decode(value)?;

       if block_details.len() == 16 {

           let mut txs = Vec::new();

           let txs_bytes = astro_format::decode(block_details[13])?;

           for tx_bytes in txs_bytes {

               let tx = Transaction::try_from(tx_bytes)?;

               txs.push(tx)

           };

           let block = Block {
               accounts_hash: block_details[0].try_into().unwrap_or(Err("Accounts hash error!")?),
               block_hash: block_details[1].try_into().unwrap_or(Err("Block hash error!")?),
               chain_id: ChainID::try_from(block_details[2]).unwrap_or(Err("Chain error!")?),
               data: block_details[3].to_vec(),
               delay_difficulty: u64::from_be_bytes(block_details[4].try_into().unwrap_or(Err("Block time error!")?)),
               delay_output: block_details[5].to_vec(),
               details_hash: block_details[6].try_into().unwrap_or(Err("Details hash error!")?),
               number: opis::Integer::try_from(block_details[7]).unwrap_or(Err("Number error!")?),
               previous_block_hash: block_details[8].try_into().unwrap_or(Err("Previous block hash error!")?),
               receipts_hash: block_details[9].try_into().unwrap_or(Err("Receipts hash error!")?),
               signature: block_details[10].try_into().unwrap_or(Err("Signature error!")?),
               solar_used: (&opis::Integer::try_from(block_details[11]).unwrap_or(Err("Block number error!")?)).into(),
               time: u64::from_be_bytes(block_details[12].try_into().unwrap_or(Err("Block time error!")?)),
               transactions: txs,
               transactions_hash: block_details[14].clone().try_into().unwrap_or(Err("Validator error!")?),
               validator: block_details[15].clone().try_into().unwrap_or(Err("Validator error!")?),
           };

           Ok(block)

       } else {

           Err("Block details error!")?

       }

   }
   
}

impl TryFrom<Vec<u8>> for Block {
   type Error = Box<dyn Error>;

   fn try_from(value: Vec<u8>) -> Result<Self, Box<dyn Error>> {
       Block::try_from(&value[..])
   }
}

impl Into<Vec<u8>> for &Block {

   fn into(self) -> Vec<u8> {

       let chain_bytes: Vec<u8> = (&self.chain_id).into();

       let delay_difficulty_bytes: Vec<u8> = opis::Integer::from(&self.delay_difficulty).into();

       let number_bytes: Vec<u8> = (&self.number).into();

       let solar_used_bytes: Vec<u8> = opis::Integer::from(&self.solar_used).into();

       let time_bytes: Vec<u8> = opis::Integer::from(&self.time).into();

       let tx_bytes: Vec<Vec<u8>> = self.transactions.iter().map(|x| x.into()).collect();

       astro_format::encode(&[
           &self.accounts_hash,
           &self.block_hash,
           &chain_bytes,
           &self.data,
           &delay_difficulty_bytes,
           &self.delay_output,
           &self.details_hash,
           &number_bytes,
           &self.previous_block_hash,
           &self.receipts_hash,
           &self.signature,
           &solar_used_bytes,
           &time_bytes,
           &astro_format::encode(
               &(tx_bytes
                   .iter()
                   .map(|x| x.as_slice())
                   .collect::<Vec<_>>()
               )
           ),
           &self.transactions_hash,
           &self.validator.0,
       ])

   }

}

impl Into<Vec<u8>> for Block {
   fn into(self) -> Vec<u8> {
       (&self).into()
   }
}
