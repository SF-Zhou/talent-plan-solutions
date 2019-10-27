use crate::{KvsEngine, KvsError, Result};
use bson;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::io::Seek;
use tempfile::TempDir;

#[derive(Serialize, Deserialize)]
enum Command {
    Insert { key: String, value: String },
    Delete { key: String },
}

/// The `KvStore` stores key-value pairs in-memory
/// Examples
/// ```rust
/// # use kvs::{KvStore, Result, KvsEngine};
/// # fn try_main() -> Result<()> {
/// let mut store = KvStore::new()?;
/// let key = String::from("key");
/// let value = String::from("value");
/// store.set(key.clone(), value.clone())?;
/// assert_eq!(store.get(key.clone())?, Some(value.clone()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    commands_num: u32,
    log: std::fs::File,
    map: HashMap<String, u64>,
    log_path: std::path::PathBuf,
}

impl KvStore {
    pub fn new() -> Result<Self> {
        KvStore::open(std::path::Path::new("."))
    }

    pub fn open(path: &std::path::Path) -> Result<Self> {
        let log_path = if path.is_dir() {
            path.join(&std::path::Path::new("local.kvs"))
        } else {
            path.to_path_buf()
        };
        let log = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(log_path.as_path())?;

        let mut commands_num = 0;
        let map = KvStore::build(&log, &mut commands_num)?;
        Ok(KvStore {
            commands_num,
            log,
            map,
            log_path,
        })
    }

    fn compact(&mut self) -> Result<()> {
        if self.commands_num < 2 * self.map.len() as u32 {
            return Ok(());
        }

        let temp_dir = TempDir::new().unwrap();
        let mut store = KvStore::open(temp_dir.path())?;

        for it in self.map.clone() {
            let key = it.0;
            let value = self.get(key.clone())?.unwrap();
            store.set(key, value)?;
        }

        if std::fs::rename(store.log_path.as_path(), self.log_path.as_path()).is_ok() {
            *self = store;
        }
        Ok(())
    }

    fn build(mut log: &std::fs::File, commands_num: &mut u32) -> Result<HashMap<String, u64>> {
        let mut map = HashMap::new();
        let mut cur: u64 = 0;
        let size = log.metadata()?.len();

        while cur < size {
            let doc = bson::decode_document(&mut log)?;
            match bson::from_bson(bson::Bson::Document(doc))? {
                Command::Insert { key, .. } => map.insert(key, cur),
                Command::Delete { key } => map.remove(&key),
            };
            cur = log.seek(io::SeekFrom::Current(0))?;
            *commands_num += 1;
        }

        Ok(map)
    }
}

impl KvsEngine for KvStore {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(offset) = self.map.get(&key) {
            self.log.seek(io::SeekFrom::Start(*offset))?;
            let doc = bson::decode_document(&mut self.log)?;
            let command: Command = bson::from_bson(bson::Bson::Document(doc))?;
            if let Command::Insert { value, .. } = command {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn set(&mut self, key: String, value: String) -> Result<()> {
        let cur = self.log.seek(io::SeekFrom::Current(0))?;
        let command = Command::Insert {
            key: key.clone(),
            value,
        };
        if let Ok(bson::Bson::Document(doc)) = bson::to_bson(&command) {
            bson::encode_document(&mut self.log, &doc)?;
            self.commands_num += 1;
        }
        self.map.insert(key, cur);
        let _ = self.compact();
        Ok(())
    }

    fn remove(&mut self, key: String) -> Result<()> {
        if self.map.get(&key).is_none() {
            return Err(KvsError::KeyNotFound);
        }
        let command = Command::Delete { key: key.clone() };
        if let Ok(bson::Bson::Document(doc)) = bson::to_bson(&command) {
            bson::encode_document(&mut self.log, &doc)?;
            self.commands_num += 1;
        }
        self.map.remove(&key);
        let _ = self.compact();
        Ok(())
    }
}
