use std::{time::Instant, thread, error::Error, collections::HashMap, sync::{Arc, Mutex, RwLock}, rc::Rc, cell::RefCell};

use super::{App, address::Address, account::Account, transaction::Transaction, receipt::Receipt};

impl App {

   pub fn current_validator(&self) -> Result<Address, Box<dyn Error>> {
      todo!()
   }

   pub fn mine(&self, account_key: &[u8;32]) -> Result<(), Box<dyn Error>> {

      let mining_address = Address(fides::ed25519::public_key(account_key)?);

      let pending_transactions_pointer = self.pending_transactions_pointer.clone();

      let latest_block_pointer = self.latest_block_pointer.clone();

      thread::spawn(move || {

         let mut now = Instant::now();

         loop {

            if Instant::now().duration_since(now).as_millis() > 1000 {

               now = Instant::now();

               let latest_block = match latest_block_pointer.lock() {
                  Ok(res) => res,
                  Err(_) => continue,
               };

               let current_validator = Address([0_u8;32]);
               
               if mining_address == current_validator {

                  let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

                  let mut solar_used = 0;

                  let mut transactions: Vec<Transaction> = Vec::new();

                  let mut receipts: Vec<Receipt> = Vec::new();

                  match pending_transactions_pointer.lock() {

                     Ok(pending_transactions) => {

                        for (_transaction_hash, transaction) in pending_transactions.iter() {

                           // let sender = match changed_accounts.get(&transaction.sender) {
                           //    Some(res) => res.clone(),
                           //    None => {
                           //       match self.get_account(&latest_block.accounts, transaction.sender) {
                           //          Ok(res) => res,
                           //          Err(_) => continue,
                           //    }
                           //    }
                           // };

                        //    let recipient = match changed_accounts.get(&transaction.recipient) {
                        //       Some(res) => res.clone(),
                        //       None => {
                        //          match self_arc.get_account(&latest_block.accounts, transaction.recipient) {
                        //             Ok(res) => res,
                        //             Err(_) => continue,
                        //         }
                        //       }
                        //    };

                        //    match transaction.application(&mut changed_accounts, recipient, sender) {

                        //       Ok(receipt) => {
                        //          solar_used += receipt.solar_used;
                        //          transactions.push(transaction.clone());
                        //          receipts.push(receipt);
                        //       },
                        //       Err(_) => (),
                        //    }
                        }
                     }
                  Err(_) => todo!(),
                  }
                  // broadcast block
               }
            }
         }

      });

      Ok(())
      
   }

}
