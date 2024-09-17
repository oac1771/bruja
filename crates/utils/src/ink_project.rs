use hex::FromHexError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InkProject {
    source: Source,
    spec: Spec,
}

impl InkProject {
    pub fn code(&self) -> Result<Vec<u8>, InkProjectError> {
        self.source.wasm()
    }

    pub fn get_constructor_selector(&self, constructor: &str) -> Result<Vec<u8>, InkProjectError> {
        let constructor = self
            .spec
            .constructors
            .iter()
            .find(|con| con.label == constructor)
            .ok_or_else(|| InkProjectError::ConstructorNotFound {
                val: constructor.to_string(),
            })?;

        let bytes = hex::decode(&constructor.selector[2..])?;

        Ok(bytes)
    }
}

#[derive(Deserialize, Debug)]
struct Source {
    wasm: String,
}

impl Source {
    fn wasm(&self) -> Result<Vec<u8>, InkProjectError> {
        let bytes = hex::decode(&self.wasm[2..])?;
        Ok(bytes)
    }
}

#[derive(Deserialize, Debug)]
struct Spec {
    constructors: Vec<Constructor>,
}

#[derive(Deserialize, Debug)]
struct Constructor {
    label: String,
    selector: String,
}

#[derive(Debug, thiserror::Error)]
pub enum InkProjectError {
    #[error("Hex Decode Error: {source}")]
    HexDecodeError {
        #[from]
        source: FromHexError,
    },

    #[error("Constructor not found: {val}")]
    ConstructorNotFound { val: String },
}
