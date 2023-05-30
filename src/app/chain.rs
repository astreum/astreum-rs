use std::{fmt, error::Error, collections::BTreeMap, net::IpAddr};

use super::{address::Address, block::Block};

pub struct Chain {
    pub accounts: BTreeMap<Address,[u8;32]>,
    pub latest_block: Block,
    pub error_block: [u8;32],
    pub previous_blocks: Vec<[u8;32]>,
    pub peers: Vec<IpAddr>
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