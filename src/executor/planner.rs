use crate::{executor::DbMigration, migration::Migration};

pub enum MigrationPlan {
    Pending(Migration),
}

pub fn plan(
    disk_migrations: Vec<Migration>,
    db_migrations: Vec<DbMigration>,
) -> Vec<MigrationPlan> {
    disk_migrations
        .into_iter()
        .filter(|m| !db_migrations.iter().any(|dm| dm.id == m.id))
        .map(|m| MigrationPlan::Pending(m))
        .collect()
}
