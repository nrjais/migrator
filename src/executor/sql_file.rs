use crate::traits::Executor;
use crate::Result;
use parking_lot::Mutex;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Debug)]
pub struct SqlFileBackend {
    file: Mutex<File>,
}

impl Default for SqlFileBackend {
    fn default() -> Self {
        Self::new("out.sql").expect("failed to open default sql output file")
    }
}

impl SqlFileBackend {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;

        Ok(Self {
            file: Mutex::new(file),
        })
    }

    pub fn write_query(&self, query: String) -> Result<()> {
        let mut file = self.file.lock();
        file.write(query.as_bytes())?;
        Ok(())
    }
}

impl Executor for SqlFileBackend {
    fn execute(&self, query: String) -> Result<()> {
        self.write_query(query)
    }
}
