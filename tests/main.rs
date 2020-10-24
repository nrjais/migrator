use anyhow::Result;
use assert_cmd::prelude::*;
use postgres::{Client, NoTls};
use predicates::prelude::*;
use std::path::PathBuf;
use std::{fs, process::Command};
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
    let test_dir_iter = WalkDir::new("tests/integration")
        .max_depth(1)
        .min_depth(1)
        .into_iter();

    for entry in test_dir_iter {
        init()?;
        run_migration_in(entry?.into_path())?;
    }
    Ok(())
}

fn run_migration_in(dir: PathBuf) -> Result<()> {
    let migrations_dir_iter = WalkDir::new(&dir)
        .max_depth(1)
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir());

    for entry in migrations_dir_iter {
        let migrations_dir = entry?.into_path();
        println!("# Running :- {:?}", &migrations_dir);

        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("up").current_dir(migrations_dir).assert().success();

        let mut cmd = Command::new("pg_dump");
        cmd.args(&[
            "-h",
            "localhost",
            "-U",
            "postgres",
            "-n",
            "public",
            "-O",
            "--no-tablespaces",
            "-s",
        ])
        .env("PGPASSWORD", "password");

        if std::env::var("UPDATE_SCHEMA").is_ok() {
            cmd.stdout(
                fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(dir.as_path().join("schema.sql"))?,
            )
            .spawn()?
            .wait()?;
            continue;
        }
        let schema = fs::read_to_string(dir.as_path().join("schema.sql"))?;
        cmd.assert()
            .success()
            .stdout(predicate::eq(schema.as_str()));
    }
    Ok(())
}
