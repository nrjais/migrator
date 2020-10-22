use anyhow::Result;
use assert_cmd::prelude::*;
use postgres::{Client, NoTls};
use std::{path::PathBuf, process::Command};
use walkdir::WalkDir;

fn init() -> Result<()> {
    let mut client = Client::connect("host=localhost user=postgres password=password", NoTls)?;
    client.batch_execute(
        "DROP SCHEMA IF EXISTS public CASCADE;
                CREATE SCHEMA public;",
    )?;
    Ok(())
}

#[test]
fn test_migrate_up() -> Result<()> {
    for entry in WalkDir::new("tests/integration").max_depth(1).min_depth(1) {
        init()?;
        run_migration_in(entry?.into_path())?;
    }
    Ok(())
}

fn run_migration_in(dir: PathBuf) -> Result<()> {
    let migrations_dir = WalkDir::new(dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir());

    for entry in migrations_dir {
        let dir = entry?.into_path();
        println!("# Running :- {:?}", &dir);

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("up").current_dir(dir).assert().success();
    }
    Ok(())
}
