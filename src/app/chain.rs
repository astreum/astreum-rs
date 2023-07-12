use std::{fmt, error::Error, collections::BTreeMap};

use crate::storage::Storage;

use super::{address::Address, block::{Block, BlockError}, App};

#[derive(Clone, Debug)]
pub struct Chain {
    pub block_error: Option<BlockError>,
    pub first_block: Block,
    pub latest_block: Block,
}

impl Chain {
    pub fn new(block: Block) -> Chain {
        Chain {
            block_error: None,
            first_block: block.clone(),
            latest_block: block
        }
    }
}

impl Storage {
    pub fn is_part(&self, block_hash: &[u8;32], chain: &Chain) -> Result<bool, Box<dyn Error>> {
        let mut previous_block = chain.latest_block.previous_block;
        while previous_block != [0_u8;32] {
            if &previous_block == block_hash {
                return Ok(true)
            } else {
                previous_block = self.get_index(1, &previous_block)?.hash()
            }
        }
        Ok(false)
    }
}
#[derive(Clone)]
pub enum ChainID {
    Main,
    Test
}

impl fmt::Debug for ChainID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChainID::Main => write!(f, "main"),
            ChainID::Test => write!(f, "test"),
        }
    }
}

impl TryFrom<&str> for ChainID {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "main" => Ok(ChainID::Main),
            "test" => Ok(ChainID::Test),
            _ => Err("Unknown chain option!")?
        }
    }
}

impl TryFrom<&[u8]> for ChainID {
    type Error = Box<dyn Error>;
    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {
        match value {
            [1] => Ok(ChainID::Main),
            [0] => Ok(ChainID::Test),
            _ => Err("Unknown chain option!")?
        }
    }
}

impl Into<Vec<u8>> for &ChainID {
    fn into(self) -> Vec<u8> {
        match self {
            ChainID::Main => vec![1],
            ChainID::Test => vec![0]
        }
    }
}