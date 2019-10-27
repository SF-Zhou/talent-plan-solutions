use crate::{KvsError, Request, Response, Result};
use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let tcp_reader = TcpStream::connect(addr)?;
        let tcp_writer = tcp_reader.try_clone()?;
        Ok(KvsClient {
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),
            writer: BufWriter::new(tcp_writer),
        })
    }

    fn call(&mut self, cmd: &Request) -> Result<Option<String>> {
        serde_json::to_writer(&mut self.writer, cmd)?;
        self.writer.flush()?;
        let resp = Response::deserialize(&mut self.reader)?;
        match resp {
            Response::Ok(value) => Ok(value),
            Response::Err(msg) => Err(KvsError::StringError(msg)),
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        self.call(&Request::Get { key })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.call(&Request::Set { key, value }).map(|_| ())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.call(&Request::Remove { key }).map(|_| ())
    }
}
