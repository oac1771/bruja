use codec::{Decode, Encode};

#[derive(Encode, Decode)]
pub enum Gossip {
    JobAcceptance { job_id: Vec<u8> },
}

#[derive(Encode, Decode)]
pub enum Request {
    Job(Job),
}

#[derive(Encode, Decode)]
pub struct Job {
    code: Vec<u8>,
    params: Vec<Vec<u8>>,
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

impl Job {
    pub fn new(code: Vec<u8>, params: Vec<Vec<u8>>) -> Self {
        Self { code, params }
    }
    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn params(&self) -> &Vec<Vec<u8>> {
        &self.params
    }
}
