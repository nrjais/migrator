use std::error::Error;

use backend::DummyBackend;
use executor::sql_file::SqlFileExecutor;
use migrator::Migrator;

mod backend;
mod executor;
mod migration;
mod migrator;
mod separator;
mod traits;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIGRATIONS_GLOB: &'static str = "examples/migrations/*";

fn main() -> Result<()> {
    let executor = SqlFileExecutor::default();
    let migrator = Migrator::new(DummyBackend::new(executor));
    migrator.init()?;
    migrator.migrate(MIGRATIONS_GLOB)?;
    Ok(())
}
