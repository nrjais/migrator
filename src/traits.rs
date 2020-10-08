use crate::{migration::Migration, Result};

pub trait Executor {
    fn execute(&self, query: String) -> Result<()>;
}

pub struct DbMigration {
    pub id: u32,
}

pub trait Backend {
    fn ensure_migration_table(&self) -> Result<()>;
    fn existing_migrations(&self) -> Result<Vec<DbMigration>>;
    fn migrate(&self, migration: &Migration) -> Result<()>;
}
