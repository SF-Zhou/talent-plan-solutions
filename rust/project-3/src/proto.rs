use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Get { key: String },
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}
