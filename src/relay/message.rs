use std::{net::IpAddr, error::Error};

use super::{topic::{Topic}, Relay};

#[derive(Clone, Debug)]
pub struct Message {
    pub body: Vec<u8>,
    pub topic: Topic
}

impl Into<Vec<u8>> for Message {
    
    fn into(self) -> Vec<u8> {
        (&self).into()
    }

}

impl Into<Vec<u8>> for &Message {

    fn into(self) -> Vec<u8> {

        let topic_bytes: Vec<u8> = (&self.topic).into();
        
        astro_format::encode(&[
            &self.body,
            &topic_bytes
        ])

    }

}

impl TryFrom<&[u8]> for Message {

    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {

        let message_fields = astro_format::decode(value)?;

        if message_fields.len() == 2 {

            let message = Message {
                body: message_fields[0].to_vec(),
                topic: Topic::try_from(message_fields[1])?
            };

            Ok(message)

        } else {

            Err("Internal error!")?

        }
    }
}

impl Relay {

    pub fn send_message(&self, ip_addr: IpAddr, message: Message) -> Result<(), Box<dyn Error>> {
        match self.outgoing_queue_pointer.lock() {
            Ok(mut outgoing_queue) => {
                outgoing_queue.push((ip_addr, message));
                Ok(())
            },
            Err(_) => Err("object_store lock error!")?,
        }
    }

}