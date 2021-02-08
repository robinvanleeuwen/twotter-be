use custom_error::custom_error;
use serde::Deserialize;

custom_error! {
#[derive(Deserialize)]
pub TwotterError
    MultipleResults {table: String} =
    "{{\"error\": \"{table} has multiple results, expected one.\"}}",
    RecordNotFound {table: String} =
    "{{\"error\": \"Record in {table} not found.\"}}",
    DBInsertError {table: String} =
    "{{\"error\": \"Could not insert record in table {table}\"}}",
    DBUpdateError {table: String} =
    "{{\"error\": \"Could not update record in table {table}\"}}",
    TwootMaxCharExceeded {numchar: i32} =
    "{{\"error\": \"Maximum number of characters exceeded ({numchar})\"",
    InvalidConversion {message: String} =
    "{{\"error\": \"{message}\"}}"
}