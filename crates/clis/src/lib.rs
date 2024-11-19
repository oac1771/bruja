use codec::{Decode, Encode};
use utils::services::job::Job;

#[derive(Encode, Decode)]
pub enum Gossip {
    JobAcceptance { job_id: Vec<u8> },
}

#[derive(Encode, Decode)]
pub enum Request {
    Job(Job),
}

impl Gossip {
    pub fn decode(mut msg: &[u8]) -> Result<Self, codec::Error> {
        let res = <Gossip as Decode>::decode(&mut msg)?;
        Ok(res)
    }
}

impl Request {
    pub fn decode(mut msg: &[u8]) -> Result<Self, codec::Error> {
        let res = <Request as Decode>::decode(&mut msg)?;
        Ok(res)
    }
}
