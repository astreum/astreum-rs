	use std::{error::Error, collections::{HashMap, BTreeMap}};

	use super::{block::Block, chain::Chain, transaction::Transaction, receipt::Receipt, address::Address, account::Account, object::Object};

	impl Chain {

	pub fn validate(
		&mut self,
		astreum_storage: &neutrondb::Store<[u8;32],Object>,
		accounts_store: &neutrondb::Store<Address, Account>,
		new_block: &Block
	) -> Result<(), Box<dyn Error>> {

		if new_block.previous_block_hash == self.latest_block.block_hash {

			// CHECK TIME

			// CHECK DELAY

			// CHECK VALIDATOR

			// QUERY BLOCK TRANSACTIONS

			let mut changed_accounts: HashMap<Address, Account> = HashMap::new();

			let mut solar_used = 0;

			let mut transactions: Vec<Transaction> = Vec::new();

			let mut receipts: Vec<Receipt> = Vec::new();

			for tx in &new_block.transactions {

				match tx.application(&accounts_store, &mut changed_accounts) {
				
				Ok(receipt) => {

					solar_used += receipt.solar_used;

					transactions.push(tx.clone());

					receipts.push(receipt);
				
				},
				
				Err(_) => (),

				}

			}

			let mut validator_account = Account::from_accounts(
				&new_block.validator,
				&changed_accounts,
				&accounts_store
			).unwrap();

			let validator_reward = opis::Integer::from_dec("1000000000").unwrap();

			validator_account.increase_balance(&validator_reward);

			changed_accounts.insert(new_block.validator, validator_account);

			let receipts_hashes: Vec<[u8; 32]> = receipts
				.iter()
				.map(|x| x.hash())
				.collect();

			let receipts_hash = fides::merkle_tree::root(
				fides::hash::blake_3,
				&(receipts_hashes
				.iter()
				.map(|x| x.as_slice())
				.collect::<Vec<_>>()
				)
			);

			let mut accounts_clone = self.accounts.clone();

			// update changed_accounts details hash 

			for (address, account) in &changed_accounts {
				accounts_clone.insert(*address, account.details_hash);
			}

			let accounts_hashes: Vec<_> = accounts_clone
				.iter()
				.map(|x| fides::merkle_tree::root(
				fides::hash::blake_3,
				&[&x.0.0[..], x.1])
				)
				.collect();
			
			let accounts_hash = fides::merkle_tree::root(
				fides::hash::blake_3,
				&(accounts_hashes
				.iter()
				.map(|x| x.as_slice())
				.collect::<Vec<_>>()
				)
			);

			if new_block.accounts_hash == accounts_hash {
				
				if new_block.receipts_hash == receipts_hash {
				
				if new_block.solar_used == solar_used {

					self.accounts = accounts_clone;

					self.latest_block = new_block.clone();

					Ok(())

				} else {
					Err("solar usage mismatch!")?
				}

				} else {
				Err("receipts hash mismatch!")?
				}

			} else {
				Err("accounts hash mismatch!")?
			}

		} else {
			Err("not the next block!")?
		}

	}

	}
