use std::{net::IpAddr, error::Error};

use super::{peer::Peer, Relay};



impl Relay {
	
	pub fn local_nearest(&self, hash: [u8;32]) -> Result<Option<IpAddr>, Box<dyn Error>> {


		match self.peers_pointer.lock() {
			Ok(peers) => {

				let a = opis::Integer::from(&hash[..]);

				let mut closest_option = None;

				let mut closest_distance_option = None;

				for (ip_addr, peer) in peers.iter() {

					let b = opis::Integer::from(&peer.public_key[..]);

					let peer_distance = a ^ b;

					match closest_distance_option {
						Some(closest_distance) => {

							if peer_distance < closest_distance {

								closest_option = Some(ip_addr);
								closest_distance_option = Some(peer_distance)

							}

						},
						None => {
							
							closest_option = Some(ip_addr);
							closest_distance_option = Some(peer_distance)
							
						},
					}

				}

				Ok(closest_option)

			},

			Err(_) => Err("")?,

		}
	
	}

	pub fn network_nearest(&self) -> Option<IpAddr> {
		todo!()
	}
	
}
