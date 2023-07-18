use std::error::Error;

#[derive(Clone, Debug, Copy)]
pub enum Topic {
    ObjectRequest,
    ObjectResponse,
    Ping,
    Route,
    RouteRequest,
    Transaction,
}

impl Into<Vec<u8>> for Topic {
    fn into(self) -> Vec<u8> {
        (&self).into()
    }
}

impl Into<Vec<u8>> for &Topic {

    fn into(self) -> Vec<u8> {

        match self {
            Topic::ObjectRequest => vec![0],
            Topic::ObjectResponse => vec![1],
            Topic::Ping => vec![2],
            Topic::Route => vec![3],
            Topic::RouteRequest => vec![4],
            Topic::Transaction => vec![5],
        }

    }

}

impl TryFrom<&[u8]> for Topic {

    type Error = Box<dyn Error>;

    fn try_from(value: &[u8]) -> Result<Self, Box<dyn Error>> {

        match value {
            [0] => Ok(Topic::ObjectRequest),
            [1] => Ok(Topic::ObjectResponse),
            [2] => Ok(Topic::Ping),
            [3] => Ok(Topic::Route),
            [4] => Ok(Topic::RouteRequest),
            [5] => Ok(Topic::Transaction),
            _ => Err("topic decoding error!")?
        }

    }

}
