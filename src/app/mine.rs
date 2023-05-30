use std::{time::Instant, thread, error::Error};

use super::App;

impl App {

   pub fn mine(&self) -> Result<(), Box<dyn Error>> {

      self.sync()?;

      thread::spawn(move || {

         let mut now = Instant::now();

         loop {

            if Instant::now().duration_since(now).as_millis() > 1000 {

               // check validator

               now = Instant::now()

            }
            
         }

      });

      Ok(())
      
   }

}
