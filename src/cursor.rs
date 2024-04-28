use crate::parser::Command;

pub struct TableEntry {

}

pub enum ExecResponse {
    Insert,
    Create,
    Select(Vec<TableEntry>)
}
pub fn execute_sql(query: Command) -> ExecResponse {
    // match query {
    //     Command::Select(_) => {}
    //     Command::Insert(_, _) => {}
    //     Command::Create(_, _) => {}
    // }
    ExecResponse::Create
}

// ExecResponse will return a Vector of all the selected entries
// Selected entries will be stored in a struct and have impl Display

// Create table needs to have some info stored in a hashmap of tables
// hashmap of tables will have the table name as key and some struct we define as value
// Struct we define will need to store the expected columns and their types