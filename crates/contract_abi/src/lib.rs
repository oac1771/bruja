use hex::FromHexError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Contract {
    source: Source,

    #[serde(alias = "contract")]
    meta_data: MetaData,

    pub spec: Spec,
}

#[derive(Deserialize, Serialize)]
struct Source {
    hash: String,
}

#[derive(Deserialize, Serialize)]
struct MetaData {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Spec {
    pub constructors: Vec<Constructor>,
    pub messages: Vec<Message>,
}

#[derive(Deserialize, Serialize)]
pub struct Constructor {
    label: String,
    selector: String,
    payable: bool,
}

impl Constructor {
    pub fn get_selector_bytes(&self) -> Result<Vec<u8>, FromHexError> {
        let bytes = hex::decode(&self.selector[2..])?;
        Ok(bytes)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    label: String,
    selector: String,
    payable: bool,
}

impl Message {
    pub fn get_selector_bytes(&self) -> Result<Vec<u8>, FromHexError> {
        let bytes = hex::decode(&self.selector[2..])?;
        Ok(bytes)
    }

    pub fn label(&self) -> String {
        self.label.clone()
    }
}
