use nu_engine::env::current_dir;
use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, Span,
    Spanned, SyntaxShape, Type, Value,
};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct Stor;

impl Command for Stor {
    fn name(&self) -> &str {
        "stor"
    }

    fn usage(&self) -> &str {
        "Interact with DuckDB."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["database", "query", "sql"]
    }
    fn signature(&self) -> nu_protocol::Signature {}

    fn examples(&self) -> Vec<Example> {}

    fn extra_usage(&self) -> &str {
        r#"For more glob pattern help, please refer to https://docs.rs/duckdb/latest/duckdb/"#
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {}

}
  