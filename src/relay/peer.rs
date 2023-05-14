#[derive(Clone, Debug)]
pub struct Peer {
    pub public_key: [u8; 32],
    pub validator: bool
}