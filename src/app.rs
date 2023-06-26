use crate::relay::Relay;
use crate::storage::Storage;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use self::block::Block;
use self::transaction::Transaction;
pub mod account;
pub mod address;
pub mod application;
pub mod block;
pub mod chain;
pub mod mine;
pub mod miner;
pub mod new;
pub mod receipt;
pub mod sync;
pub mod transaction;
// pub mod validate;

pub struct App {
   latest_block_pointer: Arc<Mutex<Block>>,
   pending_transactions_pointer: Arc<Mutex<HashMap<[u8;32],Transaction>>>,
   relay_pointer: Arc<Mutex<Relay>>,
   storage_pointer: Arc<Mutex<Storage>>,
}