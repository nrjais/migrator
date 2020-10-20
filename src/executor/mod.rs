use crate::{migration::Migration, Result};
use planner::{MigrationPlan, RollbackPlan};
use query_iter::QueryIter;

mod checksum;
mod planner;
mod query_iter;

#[derive(Debug, Clone)]
pub struct MigrationEntry {
    pub id: i64,
    pub checksum: String,
}

pub enum Direction {
    Up,
    Down,
}

pub trait Backend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str;
    fn execute(&mut self, query: String) -> Result<()>;
    fn db_migrations(&mut self) -> Result<Vec<MigrationEntry>>;
    fn in_transaction<'a>(
        &mut self,
        queries: impl Iterator<Item = &'a String>,
        entry: MigrationEntry,
        action: Direction,
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

    fn apply(&mut self, migration: Migration, checksum: String) -> Result<()> {
        let queries = QueryIter::new(&migration, Direction::Up);
        let entry = MigrationEntry {
            id: migration.id,
            checksum,
        };
        self.backend.in_transaction(queries, entry, Direction::Up)
    }

    pub fn init(&mut self) -> Result<()> {
        self.backend.init()
    }

    pub fn migrate(&mut self, disk_migrations: Vec<Migration>) -> Result<()> {
        let db_migrations = self.backend.db_migrations()?;
        let planned_migrations = planner::plan_up(disk_migrations, db_migrations);

        for p_migration in planned_migrations.into_iter() {
            match p_migration {
                MigrationPlan::Pending {
                    checksum,
                    migration,
                } => {
                    self.apply(migration, checksum)?;
                }
                MigrationPlan::Diverged {
                    checksum, entry, ..
                } => anyhow::bail!(
                    "Diverged migration with ID: {}, having checksum: {}, old checksum: {}",
                    entry.id,
                    checksum,
                    entry.checksum
                ),
            }
        }

        Ok(())
    }
    fn apply_down(&mut self, migration: Migration, entry: MigrationEntry) -> Result<()> {
        let queries = QueryIter::new(&migration, Direction::Down);
        self.backend
            .in_transaction(queries, entry, Direction::Down)?;
        Ok(())
    }

    pub fn rollback(&mut self, disk_migrations: Vec<Migration>) -> Result<()> {
        let db_migrations = self.backend.db_migrations()?;
        let planned_migrations = planner::plan_down(disk_migrations, db_migrations);
        for planned in planned_migrations {
            match planned {
                RollbackPlan::Pending { migration, entry } => self.apply_down(migration, entry)?,
                RollbackPlan::Diverged {
                    checksum, entry, ..
                } => anyhow::bail!(
                    "Diverged migration with ID: {}, having checksum: {}, old checksum: {}",
                    entry.id,
                    checksum,
                    entry.checksum
                ),
            }
        }
        Ok(())
    }
}
