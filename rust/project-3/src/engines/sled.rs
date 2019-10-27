use crate::{KvsEngine, KvsError, Result};
use sled::Db;

pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    pub fn new(db: Db) -> Self {
        SledKvsEngine { db }
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.set(key, value.into_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self
            .db
            .get(key)?
            .map(|v| v.to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.db.del(key)?.ok_or(KvsError::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }
}
