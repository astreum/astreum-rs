use std::{collections::HashMap, sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}}, thread, net::{UdpSocket, IpAddr, SocketAddr}, error::Error, time::{SystemTime, Instant}, str::FromStr};
use rand::Rng;

use super::{Relay, message::Message, envelope::Envelope, topic::Topic, route::{Route, RouteID}, ping::Ping};


impl Relay {

   pub fn new(seeders: Vec<IpAddr>, validator: bool) -> Result<Self, Box<dyn Error>> {

      let secret_key = fides::x25519::secret_key();

      let public_key = fides::x25519::public_key(&secret_key);

      let peer_route_pointer = Arc::new(Mutex::new(Route::new()));

      let consensus_route_pointer = Arc::new(Mutex::new(Route::new()));

      let incoming_queue_pointer = Arc::new(Mutex::new(Vec::new()));

      let incoming_socket = UdpSocket::bind("127.0.0.1:55555")?;

      let outgoing_queue_pointer = Arc::new(Mutex::new(Vec::new()));

      let peers_pointer: Arc<Mutex<HashMap<IpAddr, u64>>> = Arc::new(Mutex::new(HashMap::new()));

      let (sender, receiver): (Sender<(IpAddr, Message)>, Receiver<(IpAddr, Message)>) = channel();

      let receiver_pointer = Arc::new(Mutex::new(receiver));

      // receive bytes

      let t1_in_q_pt = incoming_queue_pointer.clone();

      thread::spawn(move || {

         let mut envelope_buffer = [0; 32000];
 
         loop {
 
             match incoming_socket.recv_from(&mut envelope_buffer) {
 
                 Ok((envelope_buffer_length, sender_socket_address)) => {
 
                     let envelope_buffer = &mut envelope_buffer[..envelope_buffer_length];
 
                     match t1_in_q_pt.lock() {
 
                         Ok(mut incoming_queue) => {
 
                             incoming_queue.push((sender_socket_address.ip(), envelope_buffer.to_vec()))
 
                         },
 
                         Err(_) => (),
 
                     }
 
                 },
 
                 Err(_) => (),
 
             }
             
         }
     
      });

      let ping = Ping { public_key, validator };

      let ping_message = Message { body: ping.into(), topic: Topic::Ping, };

      // decode received bytes

      let t2_peers_pt = peers_pointer.clone();

      let t2_in_q_pt = incoming_queue_pointer.clone();

      let t2_out_q_pt = outgoing_queue_pointer.clone();

      let t2_ping_message = ping_message.clone();

      let t2_peers_rt_pt = peer_route_pointer.clone();

      let t2_cs_rt_pt = consensus_route_pointer.clone();

      thread::spawn(move || {

         loop {

            match t2_in_q_pt.lock() {

               Ok(mut incoming_queue) => {
      
                  match incoming_queue.pop() {
                     
                     Some((ip_address, envelope_buffer)) => {
      
                        match Envelope::try_from(&envelope_buffer[..]) {
                           
                           Ok(envelope) => {

                              // check envelope validity

                              match envelope.message.topic {

                                 Topic::Object => { let _s = sender.send((ip_address, envelope.message)); },

                                 Topic::ObjectRequest => { let _s = sender.send((ip_address, envelope.message)); },

                                 Topic::Ping => {

                                    match t2_peers_pt.lock() {

                                       Ok(mut peers) => {

                                          match peers.get(&ip_address) {
                  
                                             Some(_) => (),

                                             None => {

                                                match t2_out_q_pt.lock() {
                                                   
                                                   Ok(mut outgoing_queue) => {

                                                      outgoing_queue.push((ip_address, t2_ping_message.clone()));

                                                   },
                                                   
                                                   Err(_) => (),

                                                }

                                                peers.insert(
                                                   ip_address,
                                                   SystemTime::now()
                                                      .duration_since(SystemTime::UNIX_EPOCH)
                                                      .unwrap()
                                                      .as_secs()
                                                );

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

                                          match t2_out_q_pt.lock() {
                                             
                                             Ok(mut outgoing_queue) => {

                                                for sample_ip_address in sample_socket_addresses {
   
                                                   outgoing_queue.push((sample_ip_address, t2_ping_message.clone()));
         
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

                                                match t2_peers_rt_pt.lock() {
                                                   Ok(peers_route) => peers_route.samples(),
                                                   Err(_) => Vec::new(),
                                                }

                                             },
                                             
                                             RouteID::Consensus => {

                                                match t2_cs_rt_pt.lock() {
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
                                          
                                          match t2_out_q_pt.lock() {
                                             Ok(mut outgoing_queue) => outgoing_queue.push((ip_address, route_message)),
                                             Err(_) => (),
                                          }

                                       },
                                      
                                       Err(_) => (),

                                    }
                                 
                                 },

                                 Topic::State => { let _s = sender.send((ip_address, envelope.message)); },

                                 Topic::StateRequest => { let _s = sender.send((ip_address, envelope.message)); },

                                 Topic::Transaction => { let _s = sender.send((ip_address, envelope.message)); }

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

      // send messages

      let t3_out_q_pt = outgoing_queue_pointer.clone();

      let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

      let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

      let outgoing_socket = UdpSocket::bind(outgoing_address)?;

      thread::spawn(move || {

         loop {
            
            match t3_out_q_pt.lock() {
               
               Ok(mut outgoing_queue) => {
      
                  match outgoing_queue.pop() {
                     
                     Some((outgoing_ip_address, outgoing_message)) => {

                        let outgoing_envelope = Envelope::new(outgoing_message);

                        let outgoing_envelope_bytes: Vec<u8> = outgoing_envelope.into();

                        let outgoing_socket_address = SocketAddr::new(outgoing_ip_address, 55555);

                        let _s = outgoing_socket.send_to(&outgoing_envelope_bytes, outgoing_socket_address);

                     },
                     
                     None => (),

                  }

               },
               
               Err(_) => (),
            
            }
            
         }

      });

      // check liveness

      let t4_peers_pt = peers_pointer.clone();

      let t4_peers_rt_pt = peer_route_pointer.clone();

      let t4_cs_rt_pt = consensus_route_pointer.clone();

      let t4_out_q_pt = outgoing_queue_pointer.clone();

      let t4_ping_message = ping_message.clone();

      let mut t4_time = Instant::now();

      thread::spawn(move || {

         loop {

            if Instant::now().duration_since(t4_time).as_secs() > 30 {

               match t4_peers_pt.lock() {
                  
                  Ok(mut peers) => {

                     match t4_out_q_pt.lock() {

                        Ok(mut outgoing_queue) => {

                           if peers.is_empty() {

                              for seeder in &seeders {
         
                                 outgoing_queue.push((*seeder, t4_ping_message.clone()));
                     
                              }

                           } else {

                              let current_time = SystemTime::now()
                                 .duration_since(SystemTime::UNIX_EPOCH)
                                 .unwrap()
                                 .as_secs();

                              let mut removable_peers = Vec::new();

                              for (ip_address, timestamp) in &*peers {
                           
                                 if (current_time - timestamp) > 330 {
                     
                                    removable_peers.push(ip_address.clone());

                                    match t4_peers_rt_pt.lock() {
                                       Ok(mut peer_route) => peer_route.remove(&ip_address),
                                       Err(_) => todo!(),
                                    }

                                    match t4_cs_rt_pt.lock() {
                                       Ok(mut consensus_route) => consensus_route.remove(&ip_address),
                                       Err(_) => todo!(),
                                    }                                 
                     
                                 }
                     
                                 if (current_time - timestamp) > 300 {
                                    
                                    outgoing_queue.push((*ip_address, t4_ping_message.clone()));

                                 }
                     
                              }
                     
                              for removable in removable_peers {
                     
                                 peers.remove(&removable);
                     
                              }

                           }

                        }
                     
                        Err(_) => (),
                        
                     }

                  },
                  
                  Err(_) => (),
               
               }

               t4_time = Instant::now();

            }      
         
         }

      });

      let relay = Relay {
         routes: HashMap::new(),
         peers: HashMap::new(),
         secret_key,
         public_key,
         validator,
         incoming_queue_pointer,
         outgoing_queue_pointer,
         peer_route_pointer,
         consensus_route_pointer,
         receiver_pointer,
      };

      Ok(relay)

   }

}
