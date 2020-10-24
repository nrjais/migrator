use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Migration {
    pub id: i64,
    #[serde(rename = "change")]
    pub changes: Vec<ChangeSet>,
}

#[derive(Deserialize, Debug)]
pub struct ChangeSet {
    pub up: Change,
    pub down: Option<Change>,
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
