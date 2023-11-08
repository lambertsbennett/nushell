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
        "stor cloud-init"
    }

    fn signature(&self) -> Signature {
        Signature::build("stor cloud-init")
            .input_output_types(vec![(Type::String, Type::Nothing)])
            .required(
                "cloud provider",
                SyntaxShape::String,
                "the name of the cloud to connect to.",
            )
            .named(
                "conn_str",
                SyntaxShape::String,
                "optional connection string for private storage",
                Some("c"),
            )
            .category(Category::Custom("database".into()))
    }

    fn usage(&self) -> &str {
        "Initialize Azure or AWS connection for querying."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["Azure", "AWS", "cloud", "query"]
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
        let conn_type = call.req(engine_state, stack, 0)?;
        let conn_str: Option<Spanned<String>> = call.get_flag(engine_state, stack, "conn_str")?;
        db.init_cloud(conn_type, conn_str, span)
            .map(IntoPipelineData::into_pipeline_data)
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "stor cloud-init azure",
                description: "Initialize connection to Azure storage to query public data",
                result: None,
            },
            Example {
                example: "stor cloud-init azure --conn_str <some_connection_url>",
                description: "Initialize connection to Azure storage to query private data",
                result: None,
            },
            Example {
                example: "stor cloud-init aws",
                description: "Initialize connection to aws storage to query public data",
                result: None,
            },
            Example {
                example: "stor cloud-init aws --conn_str <some_credential_id>",
                description: "Initialize connection to AWS storage to query private data",
                result: None,
            },
        ]
    }
}
