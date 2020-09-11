use migration::Migration;
use std::{error::Error, fs};

mod migration;

const FILE_PATH: &'static str = "examples/migrations/changes/1_create_table.yml";

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::OpenOptions::new().read(true).open(FILE_PATH)?;

    let migration: Migration = serde_yaml::from_reader(file)?;

    println!("migration: {:?}", migration);
    Ok(())
}
