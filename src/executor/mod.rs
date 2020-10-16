use crate::{
    migration::{Change, Migration},
    Result,
};
use planner::MigrationPlan;

mod planner;

pub struct DbMigration {
    pub id: u32,
}

pub trait DBTransaction {
    fn execute(&mut self, query: String) -> Result<()>;
    fn finish(&mut self) -> Result<()>;
}

pub trait Backend {
    type Transaction: DBTransaction;
    fn execute(&mut self, query: String) -> Result<()>;
    fn transaction(&mut self) -> Result<Self::Transaction>;
    fn db_migrations(&mut self) -> Result<Vec<DbMigration>>;
}

pub struct Executor<T: Backend> {
    backend: T,
}

impl<T: Backend> Executor<T> {
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    fn execute(&mut self, migration: Migration) -> Result<()> {
        for change_set in migration.changes.into_iter() {
            match change_set.up {
                Change::Query { query } => self.apply_changes(vec![query])?,
                Change::Queries { queries } => self.apply_changes(queries)?,
                Change::SqlFile { .. } => todo!(),
            }
        }

        Ok(())
    }

    fn apply_changes(&mut self, queries: Vec<String>) -> Result<()> {
        let mut transaction = self.backend.transaction()?;
        for query in queries.into_iter() {
            transaction.execute(query)?;
        }
        Ok(())
    }

    pub fn migrate(&mut self, disk_migrations: Vec<Migration>) -> Result<()> {
        let db_migrations = self.backend.db_migrations()?;
        let planned_migrations = planner::plan(disk_migrations, db_migrations);

        for p_migration in planned_migrations.into_iter() {
            match p_migration {
                MigrationPlan::Pending(migration) => {
                    self.execute(migration)?;
                }
            }
        }

        Ok(())
    }
}
