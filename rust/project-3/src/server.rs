use crate::{KvsEngine, Request, Response, Result};
use serde_json::Deserializer;
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct KvsServer<T: KvsEngine> {
    engine: T,
}

impl<T: KvsEngine> KvsServer<T> {
    pub fn new(engine: T) -> Self {
        KvsServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.serve(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => error!("Connection failed: {}", e),
            }
        }
        Ok(())
    }

    fn response<W: Write>(response: Response, mut writer: &mut W) -> Result<()> {
        serde_json::to_writer(&mut writer, &response)?;
        writer.flush()?;
        debug!("Sent Response: {:?}", response);
        Ok(())
    }

    fn serve(&mut self, tcp: TcpStream) -> Result<()> {
        let reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>();

        for req in req_reader {
            let req = req?;
            debug!("Receive Request: {:?}", req);
            let result = match req {
                Request::Get { key } => self.engine.get(key),
                Request::Set { key, value } => self.engine.set(key, value).map(|_| None),
                Request::Remove { key } => self.engine.remove(key).map(|_| None),
            };
            Self::response(
                match result {
                    Ok(value) => Response::Ok(value),
                    Err(e) => Response::Err(format!("{}", e)),
                },
                &mut writer,
            )?;
        }
        Ok(())
    }
}
