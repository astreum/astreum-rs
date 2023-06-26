use std::{error::Error, thread, time::{Instant, SystemTime}};

use super::App;


impl Relay {

    pub fn liveness(&self) -> Result<(), Box<dyn Error>> {

        let ping_message = self.ping_message.clone();

        let peers_pointer = self.peers_pointer.clone();

        let peer_route_pointer = self.peer_route_pointer.clone();

        let consensus_route_pointer = self.consensus_route_pointer.clone();

        let outgoing_queue_pointer = self.outgoing_queue_pointer.clone();

        thread::spawn(move || {

            let mut now = Instant::now();

			loop {

				if Instant::now().duration_since(now).as_secs() > 30 {

                    match peers_pointer.lock() {
                        
                        Ok(mut peers) => {
                            
                            let current_time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            
                            let mut removable_peers = Vec::new();

                            for (peer_ip_address, peer) in &*peers {
                            
                                if (current_time - peer.timestamp) > 330 {
                    
                                    removable_peers.push(peer_ip_address.clone());

                                    match peer_route_pointer.lock() {
                                        Ok(mut peer_route) => peer_route.remove(&peer_ip_address),
                                        Err(_) => todo!(),
                                    }

                                    match consensus_route_pointer.lock() {
                                        Ok(mut consensus_route) => consensus_route.remove(&peer_ip_address),
                                        Err(_) => todo!(),
                                    }                                 
                    
                                }
                    
                                if (current_time - peer.timestamp) > 300 {

                                    match outgoing_queue_pointer.lock() {

                                        Ok(mut outgoing_queue) => {
                                            
                                            outgoing_queue.push((*peer_ip_address, ping_message.clone()))
                                        
                                        },
                                        
                                        Err(_) => (),
                                    
                                    }

                                }
                    
                            }
                    
                            for removable in removable_peers {
                    
                                peers.remove(&removable);
                    
                            }

                        },
                        
                        Err(_) => (),
                    
                    }

                    now = Instant::now();

				}      
			
			}

		});

        Ok(())

    }
    
}