use catalog::catalog::HashId;
use codec::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum Gossip {
    JobAcceptance { job_id: HashId },
}

#[derive(Encode, Decode)]
pub enum Request {
    Job {
        code: Vec<u8>,
        params: Vec<Vec<u8>>,
        func_name: Vec<u8>,
    },
    AcknowledgeJob {
        job_id: HashId,
    },
}

impl Gossip {
    pub fn decode(mut msg: &[u8]) -> Result<Self, codec::Error> {
        let res = <Gossip as Decode>::decode(&mut msg)?;
        Ok(res)
    }
}

impl Request {
    pub fn decode(mut req: &[u8]) -> Result<Self, codec::Error> {
        let res = <Request as Decode>::decode(&mut req)?;
        Ok(res)
    }
}

impl From<(Vec<u8>, Vec<Vec<u8>>, Vec<u8>)> for Request {
    fn from(value: (Vec<u8>, Vec<Vec<u8>>, Vec<u8>)) -> Self {
        Self::Job {
            code: value.0,
            params: value.1,
            func_name: value.2,
        }
    }
}
