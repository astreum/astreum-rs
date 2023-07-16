use std::{time::{Instant, SystemTime}, thread, error::Error, collections::HashMap, sync::{Arc, Mutex, RwLock}, rc::Rc, cell::RefCell};

use super::{App, address::Address, account::Account, transaction::Transaction, receipt::Receipt};

impl App {

   pub fn mine(&self, account_key: &[u8;32]) -> Result<(), Box<dyn Error>> {

      let mining_address = Address(fides::ed25519::public_key(account_key)?);
      let pending_transactions_pointer = self.pending_transactions_pointer.clone();
      let latest_block_pointer = self.latest_block_pointer.clone();
      let storage_pointer = self.storage_pointer.clone();

      thread::spawn(move || {
         let mut now = Instant::now();
         loop {
            if Instant::now().duration_since(now).as_millis() > 1000 {
               
               now = Instant::now();
               
               let latest_block = match latest_block_pointer.lock() {
                  Ok(res) => res,
                  Err(_) => continue,
               };

               let current_time = SystemTime::now()
                  .duration_since(SystemTime::UNIX_EPOCH)
                  .unwrap()
                  .as_secs();

               let current_miner = match storage_pointer.lock() {
                  Ok(storage) => match storage.miner(&latest_block, current_time) {
                     Ok(address) => address,
                     Err(_) => continue,
                  },
                  Err(_) => continue,
               };
               
               if mining_address == current_miner {

                  let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

                  let mut solar_used = 0;

                  let mut transactions: Vec<Transaction> = Vec::new();

                  let mut receipts: Vec<Receipt> = Vec::new();

                  match pending_transactions_pointer.lock() {
                     
                     Ok(pending_transactions) => {
                        
                        for (_transaction_hash, transaction) in pending_transactions.iter() {

                           // check mining time
                           
                           let sender = match changed_accounts.get(&transaction.sender) {
                              Some(account) => account.clone(),
                              None => match storage_pointer.lock() {
                                 Ok(storage) => match storage.get_account(&latest_block.hash, &transaction.sender) {
                                       Ok(account) => account,
                                       Err(_) => continue,
                                 },
                                 Err(_) => continue,
                              },
                           };
                           
                           let recipient = match changed_accounts.get(&transaction.recipient) {
                              Some(account) => account.clone(),
                              None => match storage_pointer.lock() {
                                 Ok(storage) => match storage.get_account(&latest_block.hash, &transaction.recipient) {
                                       Ok(account) => account,
                                       Err(_) => continue,
                                 },
                                 Err(_) => continue,
                              },
                           };
                           
                           match transaction.application(&mut changed_accounts, recipient, sender) {
                              Ok(receipt) => {
                                 solar_used += receipt.solar_used;
                                 transactions.push(transaction.clone());
                                 receipts.push(receipt);
                              },
                              Err(_) => (),
                           }

                        }
                     }

                     Err(_) => continue,
                  
                  }

                  // miner tx 

                  // miner receipt

                  // update accounts

                  // transactions hash

                  // receipts hash

                  // calculate vdf 

                  // build block 
                  
                  // broadcast block

                  // publish txs & receipts
               
               }
            }
         }
      });
      Ok(())
   }
}
