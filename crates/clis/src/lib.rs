use codec::Encode;

#[derive(Encode)]
pub enum Gossip {
    JobAcceptance { job_id: Vec<u8> },
}
