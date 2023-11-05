mod commands;
mod values;

use commands::add_commands_decls;

pub use values::{
    convert_db_row_to_nu_value, convert_db_value_to_nu_value, DuckDBDatabase,
};

use nu_protocol::engine::StateWorkingSet;

pub fn add_database_decls(working_set: &mut StateWorkingSet) {
    add_commands_decls(working_set);
}
