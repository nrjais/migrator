use migrations::{ChangeLog, Migration};
use std::{error::Error, fs};

pub(crate) mod migrations;
pub(crate) mod backend;

const FILE_PATH: &'static str = "examples/migrations/changelog.toml";
const MIGRATION_PATH: &'static str = "examples/migrations/changes/create_table.toml";

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read(FILE_PATH)?;

    let change_log: ChangeLog = toml::from_slice(&file)?;
    println!("change_log: {:?}", change_log);

    let file = fs::read(MIGRATION_PATH)?;
    let migration: Migration = toml::from_slice(&file)?;
    println!("migration: {:?}", migration);

    Ok(())
}
