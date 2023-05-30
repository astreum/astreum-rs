use std::{error::Error, thread, time::Instant};

use super::App;

impl App {

   pub fn sync(&self) -> Result<(), Box<dyn Error>> {

      self.incoming()?;

      self.decoding()?;

      thread::spawn(move || {

         let mut now = Instant::now();

         loop {

            if Instant::now().duration_since(now).as_millis() > 1000 {

               //  select chain

               // sync chain

               now = Instant::now();

            }
             
         }

      });

      Ok(())
      
   }

}
