use postgres::{Client, NoTls};

use crate::executor::{Backend, DBTransaction, DbMigration};
use crate::Result;

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

pub struct PostgresTransaction;

impl DBTransaction for PostgresTransaction {
    fn execute(&mut self, _: String) -> Result<()> {
        todo!()
    }

    fn finish(&mut self) -> Result<()> {
        todo!()
    }
}

impl Backend for PostgresBackend {
    type Transaction = PostgresTransaction;

    fn execute(&mut self, query: String) -> Result<()> {
        self.client.execute(query.as_str(), &[])?;
        Ok(())
    }

    fn db_migrations(&mut self) -> Result<Vec<DbMigration>> {
        todo!()
    }

    fn transaction(&mut self) -> Result<Self::Transaction> {
        todo!()
    }
}
