use std::{error::Error, net::{IpAddr, UdpSocket, SocketAddr}, sync::{Arc, Mutex}, fs::File, io::{BufReader, BufRead}, collections::HashMap};

use rand::Rng;

use crate::app::{object::Object, route::Route, ping::Ping, message::Message, topic::Topic};

use super::App;



impl App {

	pub fn new(
		account_key: [u8;32],
		validator: bool
	) -> Result<App, Box<dyn Error>> {

		let account_address = fides::ed25519::public_key(&account_key)?;

		let relay_key = fides::x25519::secret_key();

		let relay_address = fides::x25519::public_key(&relay_key);

		let objects_store: neutrondb::Store<[u8;32],Object> = neutrondb::Store::new("/neutrondb/objects")?;

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

		let ping = Ping {
			public_key: relay_address,
			validator: validator,
		};

		let ping_bytes: Vec<u8> = ping.into();

		let ping_message = Message {
			body: ping_bytes,
			topic: Topic::Ping,
		};

		// get latest block hash and get block from objects store 

		let app = App {
			validator,
			account_address,
			account_key,
			object_store_pointer: Arc::new(Mutex::new(objects_store)),
			relay_address,
			relay_key,
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
			latest_block: todo!(),
		};

		todo!()

	}

}
