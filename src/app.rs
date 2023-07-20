use crate::relay::Relay;
use crate::storage::Storage;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use self::block::Block;
use self::chain::Chain;
use self::transaction::Transaction;
pub mod account;
pub mod address;
pub mod application;
pub mod block;
pub mod chain;
pub mod mine;
pub mod new;
pub mod receipt;
pub mod sync;
pub mod transaction;
pub mod validate;

pub struct App {
   pub chains_pointer: Arc<Mutex<HashMap<[u8;32], Chain>>>,
   pub latest_block_pointer: Arc<Mutex<Block>>,
   pub pending_transactions_pointer: Arc<Mutex<HashMap<[u8;32],Transaction>>>,
   pub relay_pointer: Arc<Mutex<Relay>>,
   pub storage_pointer: Arc<Mutex<Storage>>,
}