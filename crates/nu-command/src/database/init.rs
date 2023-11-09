use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, Type,
    Value,
};

use super::super::DuckDBDatabase;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "stor init"
    }

    fn signature(&self) -> Signature {
        Signature::build("stor init")
            .input_output_types(vec![(Type::String, Type::Nothing)])
            .named(
                "file",
                SyntaxShape::Path,
                "Optional file to back DuckDB.",
                Some("f"),
            )
            .category(Category::Custom("database".into()))
    }

    fn usage(&self) -> &str {
        "Initialize DuckDB connection."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["sql", "database", "connection"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let span = call.head;
        let db = DuckDBDatabase::try_from_pipeline(input, call.head)?;
        let conn = db.open_duckdb(&db.path, span)?;
        // add db to engine state?
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "stor init",
                description: "Initialize connection to DuckDB.",
                result: None,
            },
            Example {
                example: "stor init --file duck.db",
                description: "Initialize connection to file-backed DuckDB",
                result: None,
            },
        ]
    }
}
