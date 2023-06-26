
use std::{
	error::Error,
	net::IpAddr,
	sync::{Arc, Mutex},
	fs::{File},
	io::{BufReader, BufRead},
	collections::HashMap,
	path::Path
};

use crate::storage::object::Object;

use super::{Relay, route::Route, ping::Ping, message::Message, topic::Topic};



impl Relay {

	pub fn new(
		latest_block_hash: [u8;32],
		object_store_pointer: Arc<Mutex<neutrondb::Store<[u8;32],Object>>>,
		validator: bool,
	) -> Result<Relay, Box<dyn Error>> {

		let key = fides::x25519::secret_key();

		let address = fides::x25519::public_key(&key);

		let peer_route_pointer = Arc::new(Mutex::new(Route::new()));

		let consensus_route_pointer = Arc::new(Mutex::new(Route::new()));

		let incoming_queue_pointer = Arc::new(Mutex::new(Vec::new()));

		let outgoing_queue_pointer = Arc::new(Mutex::new(Vec::new()));

		let seeders_file = File::open("./seeders.txt")?;

		let mut seeders = Vec::new();
	
		for seeder in BufReader::new(seeders_file).lines() {

			let seeder = seeder?;
			
			let seeder_ip_addr: IpAddr = seeder.parse()?;

			seeders.push(seeder_ip_addr)

		}

		let puts_queue_pointer = Arc::new(Mutex::new(Vec::new()));

		let gets_queue_pointer = Arc::new(Mutex::new(Vec::new()));

		let latest_block_path = Path::new("/latest_block.bin");

		let ping = Ping {
			public_key: address,
			validator,
    		chain: latest_block_hash,
		};

		let ping_bytes: Vec<u8> = ping.into();

		let ping_message = Message {
			body: ping_bytes,
			topic: Topic::Ping,
		};

		let app = Relay {
			validator,
			object_store_pointer,
			address,
			key,
			peer_route_pointer,
			consensus_route_pointer,
			incoming_queue_pointer,
			outgoing_queue_pointer,
			seeders,
			puts_queue_pointer,
			gets_queue_pointer,
    		storage_index: HashMap::new(),
			peers_pointer: Arc::new(Mutex::new(HashMap::new())),
			ping_message,
		};

		Ok(app)

	}

}
