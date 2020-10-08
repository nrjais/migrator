use std::convert::identity;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::migration::Migration;
use crate::separator::runnable_migrations;
use crate::{traits::Backend, Result};

pub struct Migrator<T: Backend> {
    backend: T,
}

impl<T: Backend> Migrator<T> {
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    pub fn init(&self) -> Result<()> {
        self.backend.ensure_migration_table()?;
        Ok(())
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
            .into_iter()
            .flat_map(identity)
            .map(|r| r.map_err(|e| Box::new(e) as Box<dyn Error>))
            .map(|f| f.and_then(Self::read_migration))
            .map(|r| r.ok())
            .collect::<Option<Vec<_>>>()
            .expect("failed to read files in the given directory")
    }

    fn disk_migrations(path: &str) -> Vec<Migration> {
        Self::sort_by_order(Self::read_all_migrations(path))
    }

    pub fn migrate(&self, path: &str) -> Result<()> {
        let db_migrations = self.backend.existing_migrations()?;
        let disk_migrations = Self::disk_migrations(path);

        runnable_migrations(disk_migrations, db_migrations)
            .iter()
            .for_each(|m| {
                print!("{:#?}\n", &m);
                self.backend
                    .migrate(m)
                    .expect("failed to execute migration");
            });
        Ok(())
    }
}