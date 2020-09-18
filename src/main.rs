use glob::glob;
use migration::Migration;
use std::{convert::identity, error::Error, fs, path::PathBuf};

pub(crate) mod executor;
pub(crate) mod migration;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MIGRATIONS_GLOB: &'static str = "examples/migrations/*";

fn read_migration(path: PathBuf) -> Result<Migration> {
    let migration_file = fs::read(path)?;
    let migration: Migration = toml::from_slice(&migration_file)?;
    Ok(migration)
}

fn read_all_migrations(path: &str) -> Vec<Migration> {
    glob(path)
        .into_iter()
        .flat_map(identity)
        .map(|r| r.map_err(|e| Box::new(e) as Box<dyn Error>))
        .map(|f| f.and_then(read_migration))
        .map(|r| r.ok())
        .collect::<Option<Vec<_>>>()
        .expect("failed to read files in the given directory")
}

fn main() -> Result<()> {
    let migrations = read_all_migrations(MIGRATIONS_GLOB);

    print!("{:?}", migrations);
    Ok(())
}
