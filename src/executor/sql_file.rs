use super::Backend;
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

impl SqlFileBackend {
    fn new<T>(path: T) -> Result<Self>
    where
        T: AsRef<Path>,
    {
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
        write!(*file, "{}", query)?;
        Ok(())
    }
}

impl Backend for SqlFileBackend {
    fn execute(&self, query: String) -> Result<()> {
        self.write_query(query)
    }
}
