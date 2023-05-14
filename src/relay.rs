use std::{collections::HashMap, net::IpAddr, sync::{Arc, Mutex, mpsc::Receiver}};
pub mod broadcast;
pub mod bucket;
pub mod send;
pub mod route;
pub mod ping;
pub mod message;
pub mod new;
pub mod envelope;
pub mod topic;
use route::{Route, RouteID};
use message::Message;

pub struct Relay {
   routes: HashMap<RouteID, Route>,
   peers: HashMap<IpAddr, u64>,
   secret_key: [u8;32],
   public_key: [u8;32],
   validator: bool,
   incoming_queue_pointer: Arc<Mutex<Vec<(IpAddr, Vec<u8>)>>>,
   outgoing_queue_pointer: Arc<Mutex<Vec<(IpAddr, Message)>>>,
   peer_route_pointer: Arc<Mutex<Route>>,
   consensus_route_pointer: Arc<Mutex<Route>>,
   receiver_pointer: Arc<Mutex<Receiver<(IpAddr, Message)>>>
}