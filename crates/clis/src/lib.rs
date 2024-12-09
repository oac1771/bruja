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
        who: Vec<u8>,
    },
    Result {
        result: Vec<Vec<u8>>,
        job_id: HashId,
        worker: [u8; 32],
    },
}

#[derive(Encode, Decode)]
pub enum Response {
    AcknowledgeJob { job_id: HashId },
    AcknowledgeResult { job_id: HashId },
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

    pub fn build_job_req(components: (Vec<u8>, Vec<Vec<u8>>, Vec<u8>), who: Vec<u8>) -> Self {
        Self::Job {
            code: components.0,
            params: components.1,
            func_name: components.2,
            who,
        }
    }
}

// impl Request {
//     pub fn build_job_req(components: (Vec<u8>, Vec<Vec<u8>>, Vec<u8>), who: Vec<u8>) -> Self {
//         Self::Job {
//             code: components.0,
//             params: components.1,
//             func_name: components.2,
//             who,
//         }
//     }
// }

impl Response {
    pub fn decode(mut resp: &[u8]) -> Result<Self, codec::Error> {
        let res = <Response as Decode>::decode(&mut resp)?;
        Ok(res)
    }
}
