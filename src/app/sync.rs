use std::{error::Error, thread, time::Instant, sync::mpsc::channel};

use super::App;

impl App {

   pub fn sync(&self) -> Result<(), Box<dyn Error>> {

      // self.incoming()?;

      // self.decoding()?;

      // self.outgoing()?;

      // self.liveness()?;

      thread::spawn(move || {

         loop {

            // choose valid chain 

            // check if valid chain is older than 1 sec:
            
               // check connection & reconnect
               
               // request chain info from network
             
         }

      });

      Ok(())
      
   }

}
