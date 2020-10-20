use std::fs;
use std::path::PathBuf;

use crate::migration::Migration;
use crate::{
    executor::{Backend, Executor},
    Result,
};

pub struct Migrator<T: Backend> {
    executor: Executor<T>,
}

impl<T: Backend> Migrator<T> {
    pub fn new(executor: Executor<T>) -> Self {
        Self { executor }
    }

    fn read_migration(path: PathBuf) -> Result<Migration> {
        let migration_file = fs::read(path)?;
        let migration: Migration = toml::from_slice(&migration_file)?;
        Ok(migration)
    }

    fn sort_by_order(mut migrations: Vec<Migration>) -> Vec<Migration> {
        migrations.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
        migrations
    }

    fn read_all_migrations(path: &str) -> Vec<Migration> {
        glob::glob(path)
            .expect("failed to parse glob pattern")
            .into_iter()
            .map(|f| Self::read_migration(f?) as Result<_>)
            .map(|r| r.ok())
            .collect::<Option<Vec<_>>>()
            .expect("failed to read files in the given directory")
    }

    fn disk_migrations(path: &str) -> Vec<Migration> {
        Self::sort_by_order(Self::read_all_migrations(path))
    }

    pub fn migrate(&mut self, path: &str) -> Result<()> {
        self.executor.init()?;
        self.executor.migrate(Self::disk_migrations(path))
    }

    pub fn rollback(&mut self, path: &str) -> Result<()> {
        self.executor.init()?;
        self.executor.rollback(Self::disk_migrations(path))
    }
}
