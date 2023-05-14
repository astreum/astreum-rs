use std::{net::IpAddr, error::Error};

use super::{Relay, message::Message};

impl Relay {

   pub fn send(&self, ip_address: IpAddr, message: Message) -> Result<(), Box<dyn Error>> {

      match self.outgoing_queue_pointer.lock() {
         
         Ok(mut outgoing_queue) => {
            
            outgoing_queue.push((ip_address, message));

            Ok(())

         },
        
         Err(_) => Err("outgoing queue pointer lock error!")?
    
      }

   }

}
