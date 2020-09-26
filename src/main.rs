use backend::DummyBackend;
use executor::sql_file::SqlFileBackend;
use glob::glob;
use migration::Migration;
use std::{convert::identity, error::Error, fs, path::PathBuf};
use traits::Backend;

pub(crate) mod backend;
pub(crate) mod executor;
pub(crate) mod migration;
pub(crate) mod traits;

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

fn sort_by_order(mut migrations: Vec<Migration>) -> Vec<Migration> {
    migrations.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    migrations
}

fn main() -> Result<()> {
    let executor = SqlFileBackend::default();
    let mut backend = DummyBackend::new(executor);
    backend.ensure_migration_table()?;

    let sorted_migrations = sort_by_order(read_all_migrations(MIGRATIONS_GLOB));

    let db_migrations = backend.existing_migrations()?;

    let new_migrations = unrun_migrations(sorted_migrations, db_migrations);

    new_migrations
        .iter()
        .map(|m| {
            print!("{:#?}\n", &new_migrations);
            m
        })
        .for_each(|m| {
            backend.migrate(m).expect("failed to execute migration");
        });

    Ok(())
}

fn unrun_migrations(
    sorted_migrations: Vec<Migration>,
    db_migrations: Vec<traits::DbMigration>,
) -> Vec<Migration> {
    sorted_migrations
        .into_iter()
        .filter(|m| !db_migrations.iter().any(|dm| dm.id == m.id))
        .collect()
}
