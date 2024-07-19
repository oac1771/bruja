use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Contract {
    source: Source,

    #[serde(alias = "contract")]
    meta_data: MetaData,

    spec: Spec
}

#[derive(Deserialize, Serialize)]
struct Source {
    hash: String
}

#[derive(Deserialize, Serialize)]
struct MetaData {
    name: String
}

#[derive(Deserialize, Serialize)]
struct Spec {
    constructors: Vec<Constructor>
}

#[derive(Deserialize, Serialize)]
struct Constructor {
    label: String,
    selector: String,
    payable: bool,
    mutates: bool
}