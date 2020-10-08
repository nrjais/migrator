use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use parking_lot::Mutex;

use crate::traits::Executor;
use crate::Result;

#[derive(Debug)]
pub struct SqlFileExecutor {
    file: Mutex<File>,
}

impl Default for SqlFileExecutor {
    fn default() -> Self {
        Self::new("out.sql").expect("failed to open default sql output file")
    }
}

impl SqlFileExecutor {
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
        file.write(b"\n")?;
        Ok(())
    }
}

impl Executor for SqlFileExecutor {
    fn execute(&self, query: String) -> Result<()> {
        self.write_query(query)
    }
}
