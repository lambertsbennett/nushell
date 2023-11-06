use nu_engine::get_full_help;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, IntoPipelineData, PipelineData, ShellError, Signature, Type, Value,
};

#[derive(Clone)]
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
    fn signature(&self) -> Signature {
        Signature::build("stor")
            .category(Category::Database)
            .input_output_types(vec![(Type::Nothing, Type::String)])
    }

    fn examples(&self) -> Vec<Example> {}

    fn extra_usage(&self) -> &str {
        r#"You must use one of the subcommands. For more duckdb help, please refer to https://docs.rs/duckdb/latest/duckdb/"#
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        stor(engine_state, stack, call)
    }

    fn stor(
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;

        Ok(Value::string(
            get_full_help(
                &Stor.signature(),
                &Stor.examples(),
                engine_state,
                stack,
                false,
            ),
            head,
        )
        .into_pipeline_data())
    }
}
