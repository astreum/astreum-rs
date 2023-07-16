use std::error::Error;

use crate::{app::{address::Address, block::Block}, CONSENSUS_ADDRESS};

use super::Storage;

impl Storage {
    pub fn miner(&self, block: &Block, time: u64) -> Result<Address, Box<dyn Error>> {

        if time > block.time {
      
            let time_delta = time - block.time;
      
            let misses = if time_delta > 3 {
               (time_delta - 1) / 3
            } else {
               0
            };
      
            let mut random: [u8;32] = block.delay_output[..].try_into()?;
      
            for _ in 0..misses {
               random = fides::hash::blake_3(&random);
            };
      
            let consensus_account = self.get_account(&block.accounts, &CONSENSUS_ADDRESS)?;

            let consensus_storage = self.get_pairs(&consensus_account.storage)?;
      
            let total_stake = consensus_storage
               .iter()
               .fold(opis::Integer::zero(), |acc, x| acc + x.1[..].into());
      
            let mut random_int = opis::Integer::from(&random[..]);
      
            random_int = random_int.modulo(&total_stake)?;
      
            for x in consensus_storage {
                
                random_int = random_int - x.1[..].into();
               
                if random_int <= opis::Integer::zero() {
                    return Ok(Address(x.0))
                }
      
            }
            Err("Miner Error!")?
        } else {
            Err("Block Time Error!")?
        }
    }
}