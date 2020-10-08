use crate::{
    migration::Migration,
    traits::{Backend, DbMigration, Executor},
    Result,
};

pub struct DummyBackend<T: Executor>(T);

impl<T: Executor> DummyBackend<T> {
    pub fn new(e: T) -> Self {
        Self(e)
    }
}

impl<T: Executor> Backend for DummyBackend<T> {
    fn ensure_migration_table(&self) -> Result<()> {
        self.0.execute("CREATE MIGRATION TABLE".into())?;
        Ok(())
    }

    fn existing_migrations(&self) -> Result<Vec<DbMigration>> {
        Ok(vec![
            DbMigration { id: 2 },
            DbMigration { id: 3 },
            DbMigration { id: 4 },
        ])
    }

    fn migrate(&self, migration: &Migration) -> Result<()> {
        self.0
            .execute(format!("Running migration {:?}", migration))?;
        Ok(())
    }
}
