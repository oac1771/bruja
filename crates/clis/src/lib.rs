use codec::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum Gossip {
    JobAcceptance { job_id: Vec<u8> },
}

#[derive(Encode, Decode)]
pub enum Request {
    Job { code: Vec<u8>, params: Vec<Vec<u8>> },
}

impl Gossip {
    pub fn decode(mut msg: &[u8]) -> Result<Self, codec::Error> {
        let res = <Gossip as Decode>::decode(&mut msg)?;
        Ok(res)
    }
}
