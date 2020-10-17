use super::migration_column_names;
use crate::executor::{Backend, DbMigration};
use crate::Result;
use migration_column_names::ID;
use postgres::{Client, NoTls};

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

const SELECT_ALL_MIGRATIONS_QUERY: &'static str = "SELECT ID FROM DB_CHANGELOG;";
const CHANGELOG_TABLE_CREATION_QUERY: &'static str = "
      CREATE TABLE IF NOT EXISTS DB_CHANGELOG (
        EXECUTION_ORDER serial NOT NULL,
        ID INT PRIMARY KEY NOT NULL,
        created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
      );
";

impl Backend for PostgresBackend {
    const CHANGELOG_TABLE_CREATION_QUERY: &'static str = CHANGELOG_TABLE_CREATION_QUERY;

    fn execute(&mut self, query: String) -> Result<()> {
        self.client.execute(query.as_str(), &[])?;
        Ok(())
    }

    fn db_migrations(&mut self) -> Result<Vec<DbMigration>> {
        let mut changes = Vec::new();
        for row in self.client.query(SELECT_ALL_MIGRATIONS_QUERY, &[])? {
            changes.push(DbMigration {
                id: row.try_get(ID)?,
            })
        }

        Ok(vec![])
    }

    fn in_transaction(&mut self, queries: Vec<String>) -> Result<()> {
        let mut transaction = self.client.transaction()?;
        for query in queries.iter() {
            transaction.execute(query.as_str(), &[])?;
        }
        transaction.commit()?;
        Ok(())
    }
}