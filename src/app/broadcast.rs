use std::{error::Error, net::IpAddr};

use super::{Relay, route::RouteID, message::Message};

impl Relay {

   pub fn broadcast(&self, route_id: RouteID, message: Message) -> Result<(), Box<dyn Error>> {

      let ip_addresses: Vec<IpAddr> = match route_id {
         
         RouteID::Peer => {

            match self.peer_route_pointer.lock() {
               
               Ok(peer_route) => {

                  let result: Vec<Vec<IpAddr>> = peer_route.0
                     .iter()
                     .map(|(_,x)| x.0.clone())
                     .collect();

                  result.concat()

               },

               Err(_) => Err("peer route pointer lock error!")?

            }
            
         },
        
         RouteID::Consensus => {

            match self.consensus_route_pointer.lock() {
               
               Ok(consensus_route) => {

                  let result: Vec<Vec<IpAddr>> = consensus_route.0
                     .iter()
                     .map(|(_,x)| x.0.clone())
                     .collect();

                  result.concat()

               },

               Err(_) => Err("consensus route pointer lock error!")?
               
            }
            
         },

      };

      match self.outgoing_queue_pointer.lock() {
         
         Ok(mut outgoing_queue) => {

            for ip_address in ip_addresses {
               
               outgoing_queue.push((ip_address, message.clone()))

            };

            Ok(())

         },
         
         Err(_) => Err("outgoing queue pointer lock error!")?

      }
      
   }

}
