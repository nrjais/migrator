use std::error::Error;

use backend::postgres::PostgresBackend;
use executor::Executor;
use migrator::Migrator;

mod backend;
mod executor;
mod migration;
mod migrator;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIGRATIONS_GLOB: &'static str = "examples/migrations/*";

fn main() -> Result<()> {
    let backend = PostgresBackend::default();
    let executor = Executor::new(backend);
    let mut migrator = Migrator::new(executor);
    migrator.migrate(MIGRATIONS_GLOB)?;
    Ok(())
}
