use std::{time::{Instant, SystemTime}, thread, error::Error, collections::{HashMap, BTreeMap}, sync::{Arc, Mutex, RwLock}, rc::Rc, cell::RefCell};

use crate::relay::{message::Message, topic::Topic, route::RouteID};

use super::{App, address::Address, account::Account, transaction::Transaction, receipt::Receipt, block::Block};

impl App {

   pub fn mine(&self, mining_key: [u8;32]) -> Result<(), Box<dyn Error>> {
      
      let mining_address = Address(fides::ed25519::public_key(&mining_key)?);
      let pending_transactions_pointer = self.pending_transactions_pointer.clone();
      let latest_block_pointer = self.latest_block_pointer.clone();
      let storage_pointer = self.storage_pointer.clone();
      let relay_pointer = self.relay_pointer.clone();

      thread::spawn(move || {
         
         let mut accounts: BTreeMap<[u8;32], [u8;32]> = BTreeMap::new();
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

                  let mut miner_account = match changed_accounts.get(&mining_address) {
                     Some(account) => account.clone(),
                     None => match storage_pointer.lock() {
                        Ok(storage) => match storage.get_account(&latest_block.hash, &mining_address) {
                              Ok(account) => account,
                              Err(_) => continue,
                        },
                        Err(_) => continue,
                     },
                  };

                  miner_account.balance += match opis::Integer::from_dec("1000000000000") {
                     Ok(account) => account,
                     Err(_) => continue,
                  };

                  for (changed_address, changed_account) in changed_accounts.iter_mut() {
                     changed_account.details_hash();
                     let account_hash = fides::merkle_tree::root(
                        fides::hash::blake_3,
                        &[&changed_address.0, &changed_account.details_hash]
                     );
                     accounts.insert(changed_address.into(), account_hash);
                  };

                  let accounts_hashes: Vec<&[u8]> = accounts.values().map(|v| v.as_ref()).collect();

                  let accounts_hash = fides::merkle_tree::root(
                     fides::hash::blake_3,
                     &accounts_hashes
                  );

                  let transactions_hash = fides::merkle_tree::root(
                     fides::hash::blake_3,
                     &(transactions
                        .iter()
                        .map(|x| x.hash.as_ref())
                        .collect::<Vec<_>>()
                     )
                  );

                  let receipts_hash = fides::merkle_tree::root(
                     fides::hash::blake_3,
                     &(receipts
                        .iter()
                        .map(|x| x.hash.as_ref())
                        .collect::<Vec<_>>()
                     )
                  );

                  // calculate vdf
                  
                  let mut new_block = Block {
                     accounts: accounts_hash,
                     chain_id: latest_block.chain_id.clone(),
                     data: "Astreum Foundation Node v0.0.1".as_bytes().into(),
                     delay_difficulty: latest_block.delay_difficulty,
                     delay_output: fides::hash::blake_3(&latest_block.delay_output).to_vec(),
                     hash: [0_u8;32],
                     miner: mining_address,
                     number: &latest_block.number + &opis::Integer::one(),
                     previous_block: latest_block.hash,
                     receipts: receipts_hash,
                     signature: [0_u8;64],
                     solar_used,
                     time: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                     transactions: transactions_hash,
                    details_hash: [0_u8;32],
                  };

                  new_block.details_hash();

                  let _r = new_block.signature(&mining_key);

                  new_block.hash();

                  match relay_pointer.lock() {
                     Ok(relay) =>  match relay.ping_pointer.lock() {
                        Ok(mut ping) => {
                           
                           ping.chain = new_block.hash;
                           
                           let ping_bytes: Vec<u8> = ping.clone().into();

                           let ping_message = Message {
                              body: ping_bytes,
                              topic: Topic::Ping,
                           };

                           let _r = relay.broadcast(RouteID::Peer, ping_message);

                        },
                        Err(_) => continue,
                     },
                     Err(_) => continue,
                  }

                  // publish block, accs, txs & receipts
               
               }
            }
         }
      });
      Ok(())
   }
}
