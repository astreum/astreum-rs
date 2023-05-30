use std::{sync::{Arc, Mutex}, net::{IpAddr, UdpSocket}, collections::HashMap};

use self::{object::Object, chain::Chain, route::Route, peer::Peer, message::Message};

pub mod account;
pub mod address;
pub mod application;
pub mod block;
pub mod bucket;
pub mod chain;
pub mod decoding;
pub mod envelope;
pub mod incoming;
pub mod liveness;
pub mod message;
pub mod mine;
pub mod new;
pub mod object;
pub mod outgoing;
pub mod peer;
pub mod ping;
pub mod receipt;
pub mod route;
pub mod sync;
pub mod topic;
pub mod transaction;
pub mod validate;

pub struct App {
   validator: bool,
   account_address: [u8;32],
   account_key: [u8;32],
   object_store_pointer: Arc<Mutex<neutrondb::Store<[u8;32], Object>>>,
   chains: Vec<Chain>,
   longest_chain: Chain,
   relay_address: [u8;32],
   relay_key: [u8;32],
   peer_route_pointer: Arc<Mutex<Route>>,
   consensus_route_pointer: Arc<Mutex<Route>>,
   incoming_queue_pointer: Arc<Mutex<Vec<(IpAddr,Vec<u8>)>>>,
   outgoing_queue_pointer: Arc<Mutex<Vec<(IpAddr, Message)>>>,
   seeders: Vec<IpAddr>,
   puts_queue_pointer: Arc<Mutex<Vec<[u8;32]>>>,
   gets_queue_pointer: Arc<Mutex<Vec<[u8;32]>>>,
   storage_index: HashMap<[u8;32],Vec<IpAddr>>,
   peers_pointer: Arc<Mutex<HashMap<IpAddr, Peer>>>,
   ping_message: Message
}