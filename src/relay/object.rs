
pub struct ObjectRequest {
	hash: [u8;32],
	kind: ObjectRequestKind,
 }

impl Into<Vec<u8>> for ObjectRequest {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
 
pub enum ObjectRequestKind {
	Get,
	Put
}

impl Into<Vec<u8>> for ObjectRequestKind {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
pub struct ObjectResponse{
	data: Vec<u8>,
	kind: ObjectResponseKind,
	hash: [u8;32],
	request: ObjectRequestKind
 }
 
 pub enum ObjectResponseKind { Ok, Next }
