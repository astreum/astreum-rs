mod broadcast;
mod bucket;
mod connect;
mod chain;
mod new;
mod decoding;
mod distance;
mod incoming;
mod liveness;
pub mod message;
mod object;
mod outgoing;
mod peer;
mod ping;
pub mod route;
mod start;
pub mod topic;
pub mod envelope;
use std::{sync::{Mutex, Arc}, net::IpAddr, error::Error, collections::HashMap, time, thread::sleep};

use crate::storage::object::Object;

use self::{peer::Peer, message::Message, route::Route, object::{ObjectRequest, ObjectRequestKind}, topic::Topic, ping::Ping};

pub struct Relay {
    pub object_store_pointer: Arc<Mutex<neutrondb::Store<[u8;32], Object>>>,
    pub validator: bool,
    pub address: [u8;32],
    pub key: [u8;32],
    pub peer_route_pointer: Arc<Mutex<Route>>,
    pub consensus_route_pointer: Arc<Mutex<Route>>,
    pub incoming_queue_pointer: Arc<Mutex<Vec<(IpAddr,Vec<u8>)>>>,
    pub outgoing_queue_pointer: Arc<Mutex<Vec<(IpAddr, Message)>>>,
    pub seeders: Vec<IpAddr>,
    pub puts_queue_pointer: Arc<Mutex<Vec<[u8;32]>>>,
    pub gets_queue_pointer: Arc<Mutex<Vec<[u8;32]>>>,
    pub storage_index: HashMap<[u8;32],Vec<IpAddr>>,
    pub peers_pointer: Arc<Mutex<HashMap<IpAddr, Peer>>>,
    pub ping_pointer: Arc<Mutex<Ping>>,
}

impl Relay {

    pub fn local_get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {
        match self.object_store_pointer.lock() {
            Ok(object_store) => {
                object_store.get(&object_hash)
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

    pub fn network_get_object(&self, object_hash: &[u8;32]) -> Result<Object, Box<dyn Error>> {

        let nearest_peer = self.nearest_peer(object_hash)?;

        let mut result = Err("Not Found!")?;

        let object_request = ObjectRequest {
            hash: *object_hash,
            kind: ObjectRequestKind::Get
        };
        
        let object_request_message = Message {
            body: object_request.into(),
            topic: Topic::ObjectRequest
        };

        let _c = self.send_message(nearest_peer, object_request_message.clone())?;

        for _ in 0..3 {
            for _ in 0..10 {
                sleep(time::Duration::from_millis(100));
                match self.local_get_object(object_hash) {
                    Ok(object) => {
                        result = Ok(object);
                        break;
                    },
                    Err(_) => (),
                }
            }
            match result {
                Ok(_) => break,
                Err(_) => {
                    let random_peer = self.nearest_peer(object_hash)?;
                    let _c = self.send_message(random_peer, object_request_message.clone())?;
                },
            }
        }

        result

    }

}