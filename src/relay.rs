use std::{collections::HashMap, net::IpAddr, sync::{Arc, Mutex, mpsc::Receiver}};
pub mod broadcast;
pub mod bucket;
pub mod connect;
pub mod envelope;
pub mod message;
pub mod new;
pub mod ping;
pub mod route;
pub mod send;
pub mod topic;
use route::Route;
use message::Message;

pub struct Relay {
   peers: HashMap<IpAddr, u64>,
   secret_key: [u8;32],
   public_key: [u8;32],
   validator: bool,
   incoming_queue_pointer: Arc<Mutex<Vec<(IpAddr, Vec<u8>)>>>,
   outgoing_queue_pointer: Arc<Mutex<Vec<(IpAddr, Message)>>>,
   peer_route_pointer: Arc<Mutex<Route>>,
   consensus_route_pointer: Arc<Mutex<Route>>,
   receiver_pointer: Arc<Mutex<Receiver<(IpAddr, Message)>>>,
   relay_timeout: u64,
   relay_threshold: usize,
   seeders: Vec<IpAddr>
}