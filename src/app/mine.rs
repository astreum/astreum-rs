use std::{time::Instant, thread};

use super::App;

impl App {

   pub fn mine(&self) {

      thread::spawn(move || {

         let mut now = Instant::now();

         loop {

            if Instant::now().duration_since(now).as_millis() > 1000 {

               // check validator

               now = Instant::now()

            }
            
         }

      });
      
   }

}
