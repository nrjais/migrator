pub mod sql_file;

pub trait Backend {
    fn execute(&self, query: String) -> crate::Result<()>;
}
