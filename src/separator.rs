use crate::migration::Migration;
use crate::traits;

pub fn runnable_migrations(
    disk_migrations: Vec<Migration>,
    db_migrations: Vec<traits::DbMigration>,
) -> Vec<Migration> {
    disk_migrations
        .into_iter()
        .filter(|m| !db_migrations.iter().any(|dm| dm.id == m.id))
        .collect()
}
