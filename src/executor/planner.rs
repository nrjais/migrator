use super::checksum;
use crate::{executor::MigrationEntry, migration::Migration};

pub enum MigrationPlan {
    Pending {
        checksum: String,
        migration: Migration,
    },
    Diverged {
        checksum: String,
        migration: Migration,
        entry: MigrationEntry,
    },
}

pub fn plan_up(
    disk_migrations: Vec<Migration>,
    db_migrations: Vec<MigrationEntry>,
) -> Vec<MigrationPlan> {
    disk_migrations
        .into_iter()
        .map(|m| {
            let entry = db_migrations.iter().find(|dm| dm.id == m.id);
            (m, entry)
        })
        .filter_map(verify)
        .collect()
}

fn verify((migration, entry): (Migration, Option<&MigrationEntry>)) -> Option<MigrationPlan> {
    let checksum = checksum::new(&migration);
    match entry {
        Some(entry) if entry.checksum != checksum => Some(MigrationPlan::Diverged {
            checksum,
            migration,
            entry: entry.clone(),
        }),
        Some(_) => None,
        None => Some(MigrationPlan::Pending {
            checksum,
            migration,
        }),
    }
}

pub enum RollbackPlan {
    Pending {
        migration: Migration,
        entry: MigrationEntry,
    },
    Diverged {
        checksum: String,
        migration: Migration,
        entry: MigrationEntry,
    },
}

pub(crate) fn plan_down(
    disk_migrations: Vec<Migration>,
    db_migrations: Vec<MigrationEntry>,
) -> Option<RollbackPlan> {
    let entry = db_migrations.get(db_migrations.len() - 1usize)?;
    let migration = disk_migrations.into_iter().find(|m| m.id == entry.id)?;
    let checksum = checksum::new(&migration);

    Some(if entry.checksum == checksum {
        RollbackPlan::Pending {
            migration,
            entry: entry.clone(),
        }
    } else {
        RollbackPlan::Diverged {
            checksum,
            migration,
            entry: entry.clone(),
        }
    })
}
