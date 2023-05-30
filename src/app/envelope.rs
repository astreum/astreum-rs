use std::{error::Error, time::SystemTime};

use super::message::Message;

pub struct Envelope {
   pub message: Message,
   pub nonce: u64,
   pub time: u64
}

impl Envelope {
	
	pub fn new(message: Message) -> Envelope {
		
		let time = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.unwrap()
			.as_secs();
		
		let time_bytes = time.to_be_bytes();

		let message_bytes: Vec<u8> = (&message).into();
		
		let mut nonce = 0_u64;
		
		let mut message_hash = fides::merkle_tree::root(
			fides::hash::blake_3,
			&[
				&message_bytes,
				&nonce.to_be_bytes(),
				&time_bytes
			]
		);
		
		while message_hash[0] != 0 {

			nonce += 1;

			message_hash = fides::merkle_tree::root(
				fides::hash::blake_3,
				&[
					&message_bytes,
					&nonce.to_be_bytes(),
					&time_bytes
				]
			);

	   	}

		Envelope {
			message,
			nonce,
			time
		}

   }

}

impl Into<Vec<u8>> for Envelope {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &Envelope {

   fn into(self) -> Vec<u8> {

	let message_bytes: Vec<u8> = (&self.message).into();

	   astro_format::encode(&[
		   &message_bytes,
		   &self.nonce.to_be_bytes()[..],
		   &self.time.to_be_bytes()[..],
	   ])

   }

}

impl TryFrom<&[u8]> for Envelope {

   type Error = Box<dyn Error>;

   fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

	   let envelope_fields = astro_format::decode(value)?;

	   if envelope_fields.len() == 3 {
		   
		   let envelope = Envelope {
			   message: Message::try_from(envelope_fields[0])?,
			   nonce: u64::from_be_bytes(envelope_fields[1].try_into()?),
			   time: u64::from_be_bytes(envelope_fields[2].try_into()?),
		   };

		   Ok(envelope)

	   } else {

		   Err("envelope fields error!")?

	   }

   }

}