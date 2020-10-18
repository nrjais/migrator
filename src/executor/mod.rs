use crate::{migration::Migration, Result};
use planner::MigrationPlan;
use query_iter::QueryIter;

mod planner;
mod query_iter;

pub struct MigrationEntry {
    pub id: i64,
}

pub trait Backend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str;
    fn execute(&mut self, query: String) -> Result<()>;
    fn db_migrations(&mut self) -> Result<Vec<MigrationEntry>>;
    fn in_transaction<'a>(
        &mut self,
        queries: impl Iterator<Item = &'a String>,
        entry: MigrationEntry,
    ) -> Result<()>;

    fn init(&mut self) -> Result<()> {
        self.execute(Self::CHANGELOG_TABLE_CREATION_QUERY.into())
    }
}

pub struct Executor<T: Backend> {
    backend: T,
}

impl<T: Backend> Executor<T> {
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    fn apply(&mut self, migration: Migration) -> Result<()> {
        let queries = QueryIter::new(&migration);
        let entry = MigrationEntry { id: migration.id };
        self.backend.in_transaction(queries, entry)
    }

    pub fn init(&mut self) -> Result<()> {
        self.backend.init()
    }

    pub fn migrate(&mut self, disk_migrations: Vec<Migration>) -> Result<()> {
        let db_migrations = self.backend.db_migrations()?;
        let planned_migrations = planner::plan(disk_migrations, db_migrations);

        for p_migration in planned_migrations.into_iter() {
            match p_migration {
                MigrationPlan::Pending(migration) => {
                    self.apply(migration)?;
                }
            }
        }

        Ok(())
    }
}
