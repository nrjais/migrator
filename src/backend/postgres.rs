use super::migration_column_names;
use crate::executor::{Direction, Backend, MigrationEntry};
use crate::Result;
use migration_column_names::{CHECKSUM, ID};
use postgres::{Client, NoTls, Row};

pub struct PostgresBackend {
    client: Client,
}

impl Default for PostgresBackend {
    fn default() -> Self {
        Self::new("host=localhost user=postgres password=password")
            .expect("failed to connect to postgres")
    }
}

impl PostgresBackend {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            client: Client::connect(url, NoTls)?,
        })
    }
}

const SELECT_ALL_MIGRATIONS_QUERY: &'static str = "
  SELECT ID, CHECKSUM FROM DB_CHANGELOG ORDER BY ID;
";
const CHANGELOG_TABLE_CREATION_QUERY: &'static str = "
  CREATE TABLE IF NOT EXISTS DB_CHANGELOG (
    ID BIGINT PRIMARY KEY NOT NULL,
    EXECUTION_ORDER SERIAL NOT NULL,
    CHECKSUM VARCHAR(44) NOT NULL,
    CREATED_ON TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
  );
";
const INSERT_MIGRATION_ENTRY_QUERY: &'static str = "
  INSERT INTO DB_CHANGELOG(ID, CHECKSUM) VALUES ($1, $2);
";
const REMOVE_MIGRATION_ENTRY_QUERY: &'static str = "DELETE FROM DB_CHANGELOG WHERE ID=$1;";

fn migration_from(row: Row) -> Result<MigrationEntry> {
    Ok(MigrationEntry {
        id: row.try_get(ID)?,
        checksum: row.try_get(CHECKSUM)?,
    })
}

impl Backend for PostgresBackend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str = CHANGELOG_TABLE_CREATION_QUERY;

    fn execute(&mut self, query: String) -> Result<()> {
        self.client.execute(query.as_str(), &[])?;
        Ok(())
    }

    fn db_migrations(&mut self) -> Result<Vec<MigrationEntry>> {
        let mut changes = Vec::new();
        for row in self.client.query(SELECT_ALL_MIGRATIONS_QUERY, &[])? {
            changes.push(migration_from(row)?);
        }

        Ok(changes)
    }

    fn in_transaction<'a>(
        &mut self,
        queries: impl Iterator<Item = &'a String>,
        entry: MigrationEntry,
        action: Direction,
    ) -> Result<()> {
        let mut transaction = self.client.transaction()?;
        for query in queries {
            transaction.execute(query.as_str(), &[])?;
        }

        match action {
            Direction::Up => {
                transaction.execute(INSERT_MIGRATION_ENTRY_QUERY, &[&entry.id, &entry.checksum])?;
            }
            Direction::Down => {
                transaction.execute(REMOVE_MIGRATION_ENTRY_QUERY, &[&entry.id])?;
            }
        } 

        transaction.commit()?;
        Ok(())
    }
}
