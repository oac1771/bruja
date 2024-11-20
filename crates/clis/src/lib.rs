use catalog::catalog::HashId;
use codec::{Decode, Encode};
use utils::services::job::Job;

#[derive(Encode, Decode)]
pub enum Gossip {
    JobAcceptance { job_id: HashId },
}

#[derive(Encode, Decode)]
pub enum Request {
    Job(Job),
    AcknowledgeJob { job_id: HashId },
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
