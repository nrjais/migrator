use std::error::Error;

pub mod sql_file;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Backend {
    fn execute(&self, query: String) -> Result<()>;
}
