use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Migration {
    pub id: u32,
    change: Vec<ChangeSet>,
}

#[derive(Deserialize, Debug)]
pub struct ChangeSet {
    dialect: Option<String>,
    up: Change,
    down: Option<Change>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Change {
    Query { query: String },
    Queries { queries: Vec<String> },
    SqlFile { sql_file: SqlFile },
}

#[derive(Deserialize, Debug)]
pub struct SqlFile {
    path: String,
    new_line_delimited: bool,
}
