use std::error::Error;

use crate::storage::Storage;

use super::{chain::ChainID, address::Address};

#[derive(Clone, Debug)]
pub struct Block {
   pub accounts: [u8; 32],
   pub chain_id: ChainID,
   pub data: Vec<u8>,
   pub delay_difficulty: u64,
   pub delay_output: Vec<u8>,
   pub hash: [u8; 32],
   pub miner: Address,
   pub number: opis::Integer,
   pub previous_block: [u8; 32],
   pub receipts: [u8; 32],
   pub signature: [u8; 64],
   pub solar_used: u64,
   pub time: u64,
   pub transactions: [u8; 32],
}

impl Block {

    pub fn new(chain_id: ChainID) -> Block {
        Block {
            accounts: [0_u8; 32],
            hash: [0_u8; 32],
            chain_id,
            data: Vec::new(),
            delay_difficulty: 1,
            delay_output: Vec::new(),
            number: opis::Integer::zero(),
            previous_block: [0_u8; 32],
            receipts: [0_u8; 32],
            signature: [0_u8 ;64],
            solar_used: 0,
            time: 1,
            transactions: [0_u8; 32],
            miner: Address([0_u8; 32])
        }
    }

    pub fn sign(&mut self, secret_key: &[u8; 32]) -> Result<(), Box<dyn Error>> {
        self.signature = fides::ed25519::sign(&self.details_hash(), secret_key)?;
        Ok(())
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
                &self.accounts,
                &chain_bytes,
                &self.data,
                &delay_difficulty_bytes,
                &self.delay_output,
                &self.miner.0,
                &number_bytes,
                &self.previous_block,
                &self.receipts,
                &self.signature,
                &solar_used_bytes,
                &time_bytes,
                &self.transactions,
            ]
        )
    
    }
    
    pub fn update_hash(&mut self) {
        self.hash = self.hash()
    }

    pub fn hash(&self) -> [u8; 32] {
        fides::merkle_tree::root(
            fides::hash::blake_3,
            &[
                &self.details_hash(),
                &self.signature
            ]
        )
    }

}

impl TryFrom<&[u8]> for Block {
   type Error = Box<dyn Error>;
   fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
       let block_details = astro_format::decode(value)?;
       if block_details.len() == 16 {
           let block = Block {
               accounts: block_details[0].try_into().unwrap_or(Err("Accounts hash error!")?),
               hash: [0_u8;32],
               chain_id: ChainID::try_from(block_details[2]).unwrap_or(Err("Chain error!")?),
               data: block_details[3].to_vec(),
               delay_difficulty: u64::from_be_bytes(block_details[4].try_into().unwrap_or(Err("Block time error!")?)),
               delay_output: block_details[5].to_vec(),
               miner: block_details[15].clone().try_into().unwrap_or(Err("Validator error!")?),
               number: opis::Integer::try_from(block_details[7]).unwrap_or(Err("Number error!")?),
               previous_block: block_details[8].try_into().unwrap_or(Err("Previous block hash error!")?),
               receipts: block_details[9].try_into().unwrap_or(Err("Receipts hash error!")?),
               signature: block_details[10].try_into().unwrap_or(Err("Signature error!")?),
               solar_used: (&opis::Integer::try_from(block_details[11]).unwrap_or(Err("Block number error!")?)).into(),
               time: u64::from_be_bytes(block_details[12].try_into().unwrap_or(Err("Block time error!")?)),
               transactions: block_details[14].clone().try_into().unwrap_or(Err("Validator error!")?),
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
       astro_format::encode(&[
           &self.accounts,
           &self.hash,
           &chain_bytes,
           &self.data,
           &delay_difficulty_bytes,
           &self.delay_output,
           &self.miner.0,
           &number_bytes,
           &self.previous_block,
           &self.receipts,
           &self.signature,
           &solar_used_bytes,
           &time_bytes,
           &self.transactions,
       ])
   }
}

impl Into<Vec<u8>> for Block {
   fn into(self) -> Vec<u8> {
       (&self).into()
   }
}

impl Storage {

    pub fn get_block(&self, block_hash: &[u8;32]) -> Result<Block, Box<dyn Error>> {
        let block_objects = self.object_children(block_hash)?;
        let detail_objects = self.get_list(&block_objects[0].hash())?;
        if detail_objects.len() != 12 {
            return Err("Block field error!")?;
        }
        let chain_id = ChainID::try_from(&detail_objects[1].data[..])?;
        let mut block = Block {
            accounts: detail_objects[0].hash(),
            chain_id,
            data: detail_objects[2].data.clone(),
            delay_difficulty: u64::from_be_bytes(
                detail_objects[3].data[..].try_into()?
            ),
            delay_output: detail_objects[4].data.clone(),
            hash: [0_u8;32],
            miner: Address(detail_objects[5].data[..].try_into()?),
            number: detail_objects[6].data[..].into(),
            previous_block: detail_objects[7].data[..].try_into()?,
            receipts: detail_objects[8].data[..].try_into()?,
            signature: block_objects[1].data[..].try_into()?,
            solar_used: u64::from_be_bytes(detail_objects[9].data[..].try_into()?),
            time: u64::from_be_bytes(detail_objects[10].data[..].try_into()?),
            transactions: detail_objects[11].data[..].try_into()?,
        };
        block.update_hash();
        Ok(block)
    }
}
