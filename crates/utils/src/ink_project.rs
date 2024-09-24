use hex::FromHexError;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InkProject {
    source: Source,
    spec: Spec,
    storage: Storage,
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

    pub fn get_storage_field(&self, field_name: &str) -> Result<&Field, InkProjectError> {
        let storage_field = self
            .storage
            .root
            .layout
            .structure
            .fields
            .iter()
            .find(|field| field.name == field_name)
            .ok_or_else(|| InkProjectError::StorageFieldNotFound {
                val: field_name.to_string(),
            })?;

        Ok(storage_field)
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

#[derive(Deserialize, Debug)]
struct Storage {
    root: Root,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Root {
    layout: Layout,
    root_key: String,
    ty: u8,
}

#[derive(Deserialize, Debug)]
struct Layout {
    #[serde(rename = "struct")]
    structure: Struct,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Struct {
    fields: Vec<Field>,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Field {
    layout: FieldLayout,
    name: String,
}

impl Field {
    pub fn get_storage_key(&self) -> Result<Vec<u8>, InkProjectError> {
        let root_key = &self.layout.root.root_key;
        let bytes = hex::decode(&root_key[2..])?;

        Ok(bytes)
    }
}

#[derive(Deserialize, Debug)]
struct FieldLayout {
    root: RootLayout,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct RootLayout {
    layout: LeafLayout,
    root_key: String,
    ty: u8,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct LeafLayout {
    leaf: Leaf,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Leaf {
    key: String,
    ty: u8,
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

    #[error("StorageField not found: {val}")]
    StorageFieldNotFound { val: String },
}
