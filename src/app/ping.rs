#[derive(Clone, Debug)]
pub struct Ping {
   pub chain: [u8;32],
   pub public_key: [u8;32],
   pub validator: bool
}

impl Into<Vec<u8>> for Ping {
   
   fn into(self) -> Vec<u8> {
      
      (&self).into()
   
   }
   
}

impl Into<Vec<u8>> for &Ping {

   fn into(self) -> Vec<u8> {
      
      astro_format::encode(&[
         &self.chain[..],
         &self.public_key[..],
         if self.validator {
            &[1_u8]
         } else {
            &[0_u8]
         }
      ])

   }

}

impl TryFrom<&[u8]> for Ping {

   type Error = Box<dyn std::error::Error>;

   fn try_from(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {

      let ping_fields = astro_format::decode(value)?;
      
      if ping_fields.len() == 3 {

         let ping = Ping{
            chain: ping_fields[0].try_into()?,
            public_key: ping_fields[1].try_into()?,
            validator: match ping_fields[2] {
               [0] => false,
               [1] => true,
               _ => Err("Validator details error!")?
            },
         };

         Ok(ping)
         
      } else {

         Err("Ping fields error!")?

      }

   }

}
