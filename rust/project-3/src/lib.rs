#[macro_use]
extern crate log;

mod client;
mod engines;
mod error;
mod proto;
mod server;

pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use proto::{Request, Response};
pub use server::KvsServer;
