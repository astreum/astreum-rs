use std::{time::Instant, error::Error};

use super::{Relay, message::Message, topic::Topic, route::RouteID};

impl Relay {

   pub fn connect(&self) -> Result<bool, Box<dyn Error>> {

      let peer_message = Message {
         body: RouteID::Peer.into(),
         topic: Topic::RouteRequest
      };

      let consensus_message = Message {
         body: RouteID::Consensus.into(),
         topic: Topic::RouteRequest
      };

      match self.outgoing_queue_pointer.lock() {
         
         Ok(mut outgoing_queue) => {

            for seeder in &self.seeders {

               outgoing_queue.push((*seeder, peer_message.clone()));

               outgoing_queue.push((*seeder, consensus_message.clone()))

            }

         },
        
         Err(_) => Err("outgoing queue pointer lock error!")?
    
      }

      let current_instant = Instant::now();

      while Instant::now().duration_since(current_instant).as_secs() < self.relay_timeout { }

      match self.peer_route_pointer.lock() {
         
         Ok(peer_route) => {

            match self.consensus_route_pointer.lock() {
               
               Ok(consensus_route) => {

                  let result = [peer_route,consensus_route]
                     .iter()
                     .all(|x| x.size() > self.relay_threshold);

                  Ok(result)

               },
               
               Err(_) => Err("consensus route pointer lock error!")?,
            
            }

         },
        
         Err(_) => Err("peer route pointer lock error!")?,
    
      }

   }

}
