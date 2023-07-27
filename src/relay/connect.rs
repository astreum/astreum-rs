use std::{error::Error, time::Duration, thread::sleep};

use crate::relay::{route::RouteID, message::Message, topic::Topic};

use super::Relay;

impl Relay {
    pub fn connect(&self) -> Result<bool, Box<dyn Error>> {

        for _ in 0..3 {
            
            let random_peer_ip = self.random_peer()?;
            
            let message = Message {
                body: RouteID::Peer.into(),
                topic: Topic::RouteRequest,
            };

            match self.outgoing_queue_pointer.lock() {
                Ok(mut outgoing_queue) => {outgoing_queue.push((random_peer_ip, message));},
                Err(_) => Err("Outgoing Queue Pointer Error!")?,
            }
        
            sleep(Duration::from_secs(3));

            match self.peers_pointer.lock() {
                Ok(peers) => if !peers.is_empty() {
                    return Ok(true);
                },
                Err(_) => Err("Peers Pointer Error!")?,
            }

        }

        Ok(false)

    }
}