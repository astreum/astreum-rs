use std::{error::Error, thread, time::Instant, sync::mpsc::channel, collections::HashMap};

use super::{App, block::Block, chain::Chain};

impl App {

	pub fn sync(&self) -> Result<(), Box<dyn Error>> {

		let relay_pointer = self.relay_pointer.clone();
		let storage_pointer = self.storage_pointer.clone();
		
		thread::spawn(move || {

			let mut chains: HashMap<[u8;32], Chain> = HashMap::new();

			loop {

				let network_chain_hashes = match relay_pointer.lock() {
					Ok(relay) => match relay.network_chain_hashes() {
						Ok(network_chain_hashes) => network_chain_hashes,
						Err(_) => vec![],
					},
					Err(_) => vec![],
				};

				let mut new_chains = Vec::new();

				match storage_pointer.lock() {

					Ok(storage) => {

						for network_chain_hash in network_chain_hashes {

							let latest_block = storage.get_block(&network_chain_hash).unwrap();
							
							let mut new_chain = Chain::new(latest_block);

							for (chain_hash, old_chain) in chains.iter() {

								match storage.is_part(&chain_hash, &new_chain) {
									Ok(part) => {
										match part {
											true => {
												new_chain.block_error = old_chain.block_error.clone();
												new_chain.first_block_hash = old_chain.first_block_hash;
												new_chains.push(new_chain);
												break;
											},
											false => new_chains.push(new_chain.clone()),
										}
									},
									Err(_) => new_chains.push(old_chain.clone()),
								}
							}
						}
					},
					Err(_) => (),
				}

				for new_chain in new_chains {
					// update chain 
				}

				// update ping message

			}

		});

		Ok(())
		
	}

}
