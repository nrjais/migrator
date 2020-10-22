use backend::postgres::PostgresBackend;
use executor::Executor;
use migrator::Migrator;

mod backend;
mod executor;
mod migration;
mod migrator;

pub(crate) type Result<T> = anyhow::Result<T>;

const MIGRATIONS_GLOB: &'static str = "*";

fn main() -> Result<()> {
    let cmd = std::env::args().nth(1).expect("expected command up/down");

    let backend = PostgresBackend::default();
    let executor = Executor::new(backend);
    let mut migrator = Migrator::new(executor);

    if cmd == "up" {
        migrator.migrate(MIGRATIONS_GLOB)
    } else if cmd == "down" {
        migrator.rollback(MIGRATIONS_GLOB)
    } else {
        anyhow::bail!("expected subcommand up/down")
    }
}
