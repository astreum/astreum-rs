use std::{thread, error::Error, net::{UdpSocket, SocketAddr}};

use rand::Rng;

use super::{Relay, envelope::Envelope};


impl Relay {

    pub fn outgoing(&self) -> Result<(), Box<dyn Error>> {

        let outgoing_port: u16 = rand::thread_rng().gen_range(49152..65535);

        let outgoing_address = format!("127.0.0.1:{}", outgoing_port);

        let outgoing_socket = UdpSocket::bind(outgoing_address)?;

        let outgoing_queue_pointer = self.outgoing_queue_pointer.clone();

        thread::spawn(move || {

            loop {
                
                match outgoing_queue_pointer.lock() {

                    Ok(mut outgoing_queue) => {

                        match outgoing_queue.pop() {

                            Some((outgoing_ip_address, outgoing_message)) => {

                                let outgoing_envelope = Envelope::new(outgoing_message);

                                let outgoing_envelope_bytes: Vec<u8> = outgoing_envelope.into();

                                let outgoing_socket_address = SocketAddr::new(outgoing_ip_address, 55555);

                                let _s = outgoing_socket.send_to(&outgoing_envelope_bytes, outgoing_socket_address);

                            },

                            None => (),

                        }

                    },

                    Err(_) => (),

                }

            }

        });

        Ok(())
        
    }

}