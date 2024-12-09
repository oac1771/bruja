use subxt_signer::sr25519::Keypair;

mod ink_project;
pub mod services;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

pub trait Wallet {
    fn public_key(&self) -> [u8; 32];
}

impl Wallet for Keypair {
    fn public_key(&self) -> [u8; 32] {
        self.public_key().0
    }
}
