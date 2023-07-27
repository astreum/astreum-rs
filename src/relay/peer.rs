use std::{net::IpAddr, error::Error};

use rand::{Rng, thread_rng};

use super::Relay;



pub struct Peer {
   pub public_key: [u8;32],
   pub chain: [u8;32],
   pub timestamp: u64
}

impl Relay {

   pub fn nearest_peer(&self, hash: &[u8;32]) -> Result<IpAddr, Box<dyn Error>> {

      match self.peers_pointer.lock() {
         Ok(peers) => {

            let mut closest_peer: Option<IpAddr> = None;
      
            let mut min_xor_distance: u128 = u128::MAX;
            
            for (peer_ip, peer_data) in peers.iter() {

               let peer_xor_distance = xor_distance(&peer_data.public_key, hash);
               
               if peer_xor_distance < min_xor_distance {
                  closest_peer = Some(*peer_ip);
                  min_xor_distance = peer_xor_distance;
               }
            }
            
            closest_peer.ok_or("No peers found.".into())

         },
         Err(_) => todo!(),
      }

   }

   pub fn random_peer(&self) -> Result<IpAddr, Box<dyn Error>> {

      let peer_ips: Vec<IpAddr> = match self.peers_pointer.lock() {
         Ok(peers) => peers.keys().cloned().collect(),
         Err(_) => todo!(),
      };
      
      if peer_ips.is_empty() {
          return Err("No peers available.".into());
      }
      
      let random_index = thread_rng().gen_range(0..peer_ips.len());

      Ok(peer_ips[random_index])

   }

}

fn xor_distance(a: &[u8; 32], b: &[u8; 32]) -> u128 {
   let mut result: u128 = 0;
   for i in 0..32 {
       result = (result << 8) | (a[i] ^ b[i]) as u128;
   }
   result
}
