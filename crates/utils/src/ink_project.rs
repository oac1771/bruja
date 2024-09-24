use hex::FromHexError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InkProject {
    source: Source,
    spec: Spec,
}

impl InkProject {
    pub fn code(&self) -> Result<Vec<u8>, InkProjectError> {
        let code = self.source.wasm()?;
        Ok(code)
    }

    pub fn get_constructor(&self, constructor: &str) -> Result<&Constructor, InkProjectError> {
        let constructor = self
            .spec
            .constructors
            .iter()
            .find(|con| con.label == constructor)
            .ok_or_else(|| InkProjectError::ConstructorNotFound {
                val: constructor.to_string(),
            })?;

        Ok(constructor)
    }

    pub fn get_message(&self, message: &str) -> Result<&Message, InkProjectError> {
        let message = self
            .spec
            .messages
            .iter()
            .find(|msg| msg.label == message)
            .ok_or_else(|| InkProjectError::MessageNotFound {
                val: message.to_string(),
            })?;

        Ok(message)
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
    messages: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Constructor {
    label: String,
    selector: String,
}

impl Constructor {
    pub fn get_selector(&self) -> Result<Vec<u8>, InkProjectError> {
        let bytes = hex::decode(&self.selector[2..])?;

        Ok(bytes)
    }
}

#[derive(Deserialize, Debug)]
pub struct Message {
    label: String,
    selector: String,
}

impl Message {
    pub fn get_selector(&self) -> Result<Vec<u8>, InkProjectError> {
        let bytes = hex::decode(&self.selector[2..])?;

        Ok(bytes)
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

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

    #[error("Message not found: {val}")]
    MessageNotFound { val: String },
}
