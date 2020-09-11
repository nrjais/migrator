use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ColumnType {
    Number(u8),
}

#[derive(Deserialize, Debug)]
pub struct Column {
    name: String,
    #[serde(flatten)]
    column_type: ColumnType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Change {
    CreateTable { name: String, columns: Vec<Column> },
}

#[derive(Deserialize, Debug)]
pub struct Migration {
    id: i32,
    change: Change,
}
