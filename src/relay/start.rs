use std::error::Error;

use super::Relay;

impl Relay {
    pub fn start(&self) -> Result<(), Box<dyn Error>> {
        self.incoming()?;
        self.decoding()?;
        self.outgoing()?;
        self.liveness()?;
        self.connect()?;
        Ok(())
    }
}