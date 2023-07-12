use std::{thread, error::Error};

use super::App;

impl App {
	pub fn validate(&self) -> Result<(), Box<dyn Error>> {
		let chains_pointer = self.chains_pointer.clone();
		let storage_pointer = self.storage_pointer.clone();
		thread::spawn(move || {
			loop {
				match chains_pointer.lock() {
					Ok(mut chains) => {
						for (_, chain) in chains.iter_mut() {
							match chain.block_error {
								Some(_) => (),
								None => {
									if chain.first_block.hash != [0_u8;32] {
										// check headers
										let previous_block_opt = match storage_pointer.lock() {
											Ok(storage) => match storage.get_block(&chain.first_block.previous_block) {
												Ok(block) => Some(block),
												Err(_) => None,
											},
											Err(_) => None,
										};
										match previous_block_opt {
											Some(previous_block) => {
												if previous_block.number > chain.first_block.number {
													if previous_block.time > chain.first_block.time {
														chain.first_block = previous_block
													}
												}
											},
											None => (),
										}
									} else {
										// check transactions
									}
								},
							}
						}
					},
					Err(_) => (),
				};
			}
		});
		Ok(())
	}
}