use std::{fmt::Display, marker::PhantomData};

use contract_extrinsics::{ExtrinsicOptsBuilder, InstantiateCommandBuilder, InstantiateExec};
use ink_env::Environment;
use serde::Serialize;
use sp_core::{Bytes, Decode};
use subxt::{
    config::{Config, DefaultExtrinsicParams, ExtrinsicParams},
    ext::{scale_decode::IntoVisitor, scale_encode::EncodeAsType},
    tx::Signer,
};

pub struct Client<C, E, S> {
    artifact_file: String,
    signer: S,
    _config: PhantomData<C>,
    _env: PhantomData<E>,
}

impl<C: Config, E: Environment, S: Signer<C> + Clone> Client<C, E, S>
where
    C::Hash: From<[u8; 32]> + EncodeAsType + IntoVisitor,
    C::AccountId: Display + IntoVisitor + Decode,
    <<C as Config>::ExtrinsicParams as ExtrinsicParams<C>>::Params:
        From<<DefaultExtrinsicParams<C> as ExtrinsicParams<C>>::Params>,
    E::Balance: Default + EncodeAsType + Serialize,
{
    pub fn new(artifact_file: String, signer: S) -> Self {
        Self {
            artifact_file,
            signer: signer,
            _config: PhantomData::default(),
            _env: PhantomData::default(),
        }
    }

    pub async fn deploy(&self, constructor: &str) -> <C as Config>::AccountId {
        let extrinsic_opts = self.extrinsic_opts_builder().done();

        let salt: Bytes = rand::random::<[u8; 8]>().to_vec().into();

        let instantiate_exec: InstantiateExec<C, E, S> =
            InstantiateCommandBuilder::new(extrinsic_opts)
                .constructor(constructor)
                .salt(Some(salt))
                .done()
                .await
                .unwrap();

        let address = instantiate_exec
            .instantiate(None)
            .await
            .unwrap()
            .contract_address;

        return address;
    }

    fn extrinsic_opts_builder(&self) -> ExtrinsicOptsBuilder<C, E, S> {
        ExtrinsicOptsBuilder::new(self.signer.clone()).file(Some(self.artifact_file.clone()))
    }
}
