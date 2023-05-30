use std::{error::Error, net::UdpSocket, thread};

use super::App;


impl App {

    pub fn incoming(&self) -> Result<(), Box<dyn Error>> {

        let incoming_socket = UdpSocket::bind("127.0.0.1:55555")?;

		let incoming_queue_pointer = self.incoming_queue_pointer.clone();

        thread::spawn(move || {

			let mut envelope_buffer = [0; 32000];
	
			loop {
	
				match incoming_socket.recv_from(&mut envelope_buffer) {
	
					Ok((envelope_buffer_length, sender_socket_address)) => {
	
						let envelope_buffer = &mut envelope_buffer[..envelope_buffer_length];
	
						match incoming_queue_pointer.lock() {
	
							Ok(mut incoming_queue) => {
	
								incoming_queue.push((sender_socket_address.ip(), envelope_buffer.to_vec()))
	
							},
	
							Err(_) => (),
	
						}
	
					},
	
					Err(_) => (),
	
				}
				
			}
		
		});

        Ok(())

    }

}
