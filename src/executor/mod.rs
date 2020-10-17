use crate::{
    migration::{Change, Migration},
    Result,
};
use planner::MigrationPlan;

mod planner;

pub struct DBMigration {
    pub id: i64,
}

pub trait Backend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str;
    fn execute(&mut self, query: String) -> Result<()>;
    fn db_migrations(&mut self) -> Result<Vec<DBMigration>>;
    fn in_transaction(&mut self, queries: Vec<String>) -> Result<()>;

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
        self.backend.in_transaction(queries)
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
                    self.execute(migration)?;
                }
            }
        }

        Ok(())
    }
}
