use std::{error::Error, thread, net::{IpAddr, SocketAddr}, str::FromStr, time::SystemTime};

use super::{topic::Topic, message::Message, ping::Ping, peer::Peer, Relay, envelope::Envelope, route::RouteID};

impl Relay {

    pub fn decoding(&self) -> Result<(), Box<dyn Error>> {

        let ping_pointer = self.ping_pointer.clone();

        let incoming_queue_pointer = self.incoming_queue_pointer.clone();

        let peers_pointer = self.peers_pointer.clone();

        let outgoing_queue_pointer = self.outgoing_queue_pointer.clone();

        let peer_route_pointer = self.peer_route_pointer.clone();

        let consensus_route_pointer = self.consensus_route_pointer.clone();

        thread::spawn(move || {

			loop {

				match incoming_queue_pointer.lock() {

                    Ok(mut incoming_queue) => {
            
                        match incoming_queue.pop() {
                            
                            Some((sender_ip_address, envelope_buffer)) => {
            
                                match Envelope::try_from(&envelope_buffer[..]) {
                                
                                    Ok(envelope) => {

                                        // check envelope validity

                                        match envelope.message.topic {

                                            Topic::ObjectResponse => {

                                                // if get:

                                                    // if kind is object: fill queue 

                                                    // if kind is next: confirm in queue and query next

                                                // if put:

                                                    // if kind is next: confirm in queue and query next
                                                    
                                            },

                                            Topic::ObjectRequest => {

                                                // if put:

                                                    // if in index: add to index

                                                    // if other peer is nearer: send nearer as next

                                                    // if self is nearest: index object

                                                // if get:

                                                    // if in local: send object

                                                    // if in index: send index as next

                                                    // if other peer is nearer: send nearer as next
                                            },

                                            Topic::Ping => {

                                                match peers_pointer.lock() {

                                                    Ok(mut peers) => {

                                                        match peers.get(&sender_ip_address) {
                                
                                                            Some(_) => (),

                                                            None => {

                                                                let ping = match ping_pointer.lock() {
                                                                    Ok(ping) => ping.clone(),
                                                                    Err(_) => continue,
                                                                };
                                            
                                                                let ping_bytes: Vec<u8> = ping.into();
                                            
                                                                let ping_message = Message {
                                                                    body: ping_bytes,
                                                                    topic: Topic::Ping,
                                                                };

                                                                match outgoing_queue_pointer.lock() {
                                                                
                                                                    Ok(mut outgoing_queue) => {

                                                                        outgoing_queue.push((sender_ip_address, ping_message.clone()));

                                                                    },
                                                                    
                                                                    Err(_) => (),

                                                                }

                                                                match Ping::try_from(&envelope.message.body[..]) {
                                                                    
                                                                    Ok(ping) => {

                                                                        peers.insert(
                                                                            sender_ip_address,
                                                                            Peer {
                                                                                public_key: ping.public_key,
                                                                                timestamp: SystemTime::now()
                                                                                    .duration_since(SystemTime::UNIX_EPOCH)
                                                                                    .unwrap()
                                                                                    .as_secs(),
                                                                                chain: todo!(),
                                                                            }
                                                                        );

                                                                        // add to peers route

                                                                        // add to consensus route 

                                                                    },

                                                                    Err(_) => (),

                                                                }

                                                            }

                                                        }

                                                    }

                                                    Err(_) => {

                                                    },

                                                }

                                            },

                                            Topic::Route => {

                                                match astro_format::decode(&envelope.message.body) {
                    
                                                Ok(sample_bytes) => {
            
                                                    let mut sample_socket_addresses = vec![];
            
                                                    for sample in sample_bytes {
            
                                                        match String::from_utf8(sample.to_vec()) {
            
                                                            Ok(sample_socket_address_str) => {
            
                                                            match IpAddr::from_str(&sample_socket_address_str) {
            
                                                                Ok(sample_socket_address) => sample_socket_addresses.push(sample_socket_address),
            
                                                                Err(_) => (),
            
                                                            }
            
                                                            },
            
                                                            Err(_) => (),
            
                                                        }
                                                        
                                                    }

                                                    let ping = match ping_pointer.lock() {
                                                        Ok(ping) => ping.clone(),
                                                        Err(_) => continue,
                                                    };
                                
                                                    let ping_bytes: Vec<u8> = ping.into();
                                
                                                    let ping_message = Message {
                                                        body: ping_bytes,
                                                        topic: Topic::Ping,
                                                    };

                                                    match outgoing_queue_pointer.lock() {
                                                        
                                                        Ok(mut outgoing_queue) => {

                                                            for sample_ip_address in sample_socket_addresses {
            
                                                                outgoing_queue.push((sample_ip_address, ping_message.clone()));
                    
                                                            }

                                                        },
                                                        
                                                        Err(_) => (),
                                                    
                                                    }
            
                                                },
            
                                                Err(_) => (),
            
                                                }

                                            },

                                            Topic::RouteRequest => {

                                                match RouteID::try_from(&envelope.message.body[..]) {
                                                        
                                                    Ok(route_id) => {

                                                        let samples = match route_id {
                                                            
                                                            RouteID::Peer => {

                                                                match peer_route_pointer.lock() {
                                                                    Ok(peers_route) => peers_route.samples(),
                                                                    Err(_) => Vec::new(),
                                                                }

                                                            },
                                                            
                                                            RouteID::Consensus => {

                                                                match consensus_route_pointer.lock() {
                                                                    Ok(consensus_route) => consensus_route.samples(),
                                                                    Err(_) => Vec::new(),
                                                                }

                                                            },
                                                        
                                                        };

                                                        let sample_bytes: Vec<String> = samples
                                                            .iter()
                                                            .map(|x| x.to_string())
                                                            .collect();

                                                        let sample_slices: Vec<&[u8]> = sample_bytes
                                                            .iter()
                                                            .map(|x| x.as_bytes())
                                                            .collect();
                                                        
                                                        let encoded_samples = astro_format::encode(&sample_slices);
                                                        
                                                        let route_message = Message {
                                                            body: encoded_samples,
                                                            topic: Topic::Route
                                                        };
                                                
                                                        match outgoing_queue_pointer.lock() {

                                                            Ok(mut outgoing_queue) => {
                                                                
                                                                outgoing_queue.push((sender_ip_address, route_message))
                                                            
                                                            },
                                                            
                                                            Err(_) => (),
                                                        
                                                        }

                                                    },
                                                    
                                                    Err(_) => (),

                                                }
                                            
                                            },

                                            Topic::Transaction => {

                                                // if validator:

                                                    // if not in pending transactions:

                                                        // add to pending and broadcast to consensus route 

                                            }

                                        }

                                    },

                                    Err(_) => (),

                                }

                            },
                            
                            None => (),

                        }

                    },

                    Err(_) => (),

                }
			
			}

		});


        Ok(())
        
    }

}