use crate::migration::{Change, ChangeSet, Migration};
use sha3::{Digest, Sha3_256};

pub fn new(migration: &Migration) -> String {
    let mut hasher = Sha3_256::new();
    write_migration(&mut hasher, &migration);
    base64::encode(hasher.finalize())
}

fn write_migration(hasher: &mut Sha3_256, migration: &&Migration) {
    hasher.update(migration.id.to_be_bytes());
    write_changeset(hasher, &migration.changes);
}

fn write_changeset(hasher: &mut Sha3_256, changes: &Vec<ChangeSet>) {
    for changeset in changes {
        write_change(hasher, &changeset.up);
        changeset.down.as_ref().map(|c| write_change(hasher, c));
    }
}

fn write_change(hasher: &mut Sha3_256, change: &Change) {
    match change {
        Change::Query { query } => hasher.update(query.as_bytes()),
        Change::Queries { queries } => {
            for query in queries {
                hasher.update(query.as_bytes())
            }
        }
        Change::SqlFile { .. } => todo!(),
    }
}
