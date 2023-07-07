
use crate::{app::{
	block::Block,
}, relay::Relay, storage::{object::Object, Storage}};

use std::{
	error::Error,
	sync::{Arc, Mutex},
	fs,
	collections::HashMap,
	path::Path
};

use super::{App, chain::ChainID};



impl App {

	pub fn new(
		chain_id: ChainID,
		validator: bool
	) -> Result<App, Box<dyn Error>> {

		let latest_block_path = Path::new("/latest_block.bin");

		let latest_block = if latest_block_path.is_file() {

			let latest_block_bytes = fs::read(latest_block_path)?;
			
			Block::try_from(latest_block_bytes)?

		} else {

			Block::new(chain_id)
			
		};

		let object_store: neutrondb::Store<[u8;32],Object> = neutrondb::Store::new("/neutrondb/objects")?;

		let object_store_pointer = Arc::new(Mutex::new(object_store));

		let relay = Relay::new(latest_block.hash(), object_store_pointer.clone(), validator)?;

		relay.start()?;

		let relay_pointer = Arc::new(Mutex::new(relay));

		let storage = Storage {
			object_store_pointer,
			relay_pointer: relay_pointer.clone()
		};

		let app = App {
			latest_block_pointer: Arc::new(Mutex::new(latest_block)),
			pending_transactions_pointer: Arc::new(Mutex::new(HashMap::new())),
			relay_pointer,
			storage_pointer: Arc::new(Mutex::new(storage)),
		};

		Ok(app)

	}

}
