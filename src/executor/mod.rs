use crate::{
    migration::{Change, Migration},
    Result,
};
use planner::MigrationPlan;

mod planner;

pub struct DBMigration {
    pub id: i64,
    pub desc: String,
}

pub mod migration_entry_desc {
    pub const QUERY: &'static str = "query";
    pub const QUERY_LIST: &'static str = "query_list";
}

pub struct MigrationEntry {
    pub id: i64,
    pub desc: String,
}

pub trait Backend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str;
    fn execute(&mut self, query: String) -> Result<()>;
    fn db_migrations(&mut self) -> Result<Vec<DBMigration>>;
    fn in_transaction(&mut self, queries: Vec<String>, entry: MigrationEntry) -> Result<()>;

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
        for change_set in migration.changes.into_iter() {
            match change_set.up {
                Change::Query { query } => self.apply_changes(
                    vec![query],
                    migration_entry_desc::QUERY.into(),
                    migration.id,
                )?,
                Change::Queries { queries } => self.apply_changes(
                    queries,
                    migration_entry_desc::QUERY_LIST.into(),
                    migration.id,
                )?,
                Change::SqlFile { .. } => todo!(),
            }
        }

        Ok(())
    }

    fn apply_changes(&mut self, queries: Vec<String>, desc: String, id: i64) -> Result<()> {
        self.backend
            .in_transaction(queries, MigrationEntry { id, desc })
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
