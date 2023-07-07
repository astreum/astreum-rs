use std::error::Error;

use crate::app::chain::Chain;

use super::Relay;

impl Relay {
    pub fn network_chain_hashes(&self) -> Result<Vec<[u8;32]>, Box<dyn Error>> {
        let mut chain_hashes = Vec::new();
        match self.peers_pointer.lock() {
            Ok(peers) => {
                for (_,peer) in peers.iter() {
                    if !chain_hashes.contains(&peer.chain) {
                        chain_hashes.push(peer.chain)
                    }
                }
            },
            Err(_) => Err("Peers lock error!")?,
        }
        Ok(chain_hashes)
    }
}