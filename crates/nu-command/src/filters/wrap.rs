use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    record, Category, Example, IntoInterruptiblePipelineData, IntoPipelineData, PipelineData,
    ShellError, Signature, SyntaxShape, Type, Value,
};

#[derive(Clone)]
pub struct Wrap;

impl Command for Wrap {
    fn name(&self) -> &str {
        "wrap"
    }

    fn usage(&self) -> &str {
        "Wrap the value into a column."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("wrap")
            .input_output_types(vec![
                (Type::List(Box::new(Type::Any)), Type::Table(vec![])),
                (Type::Range, Type::Table(vec![])),
                (Type::Any, Type::Record(vec![])),
            ])
            .required("name", SyntaxShape::String, "the name of the column")
            .allow_variants_without_examples(true)
            .category(Category::Filters)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let span = call.head;
        let name: String = call.req(engine_state, stack, 0)?;
        let metadata = input.metadata();

        match input {
            PipelineData::Empty => Ok(PipelineData::Empty),
            PipelineData::Value(Value::Range { .. }, ..)
            | PipelineData::Value(Value::List { .. }, ..)
            | PipelineData::ListStream { .. } => Ok(input
                .into_iter()
                .map(move |x| Value::record(record! { name.clone() => x }, span))
                .into_pipeline_data(engine_state.ctrlc.clone())
                .set_metadata(metadata)),
            PipelineData::ExternalStream { .. } => Ok(Value::record(
                record! { name => input.into_value(call.head) },
                span,
            )
            .into_pipeline_data()
            .set_metadata(metadata)),
            PipelineData::Value(input, ..) => Ok(Value::record(record! { name => input }, span)
                .into_pipeline_data()
                .set_metadata(metadata)),
        }
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Wrap a list into a table with a given column name",
                example: "[1 2 3] | wrap num",
                result: Some(Value::test_list(vec![
                    Value::test_record(record! {
                        "num" => Value::test_int(1),
                    }),
                    Value::test_record(record! {
                        "num" => Value::test_int(2),
                    }),
                    Value::test_record(record! {
                        "num" => Value::test_int(3),
                    }),
                ])),
            },
            Example {
                description: "Wrap a range into a table with a given column name",
                example: "1..3 | wrap num",
                result: Some(Value::test_list(vec![
                    Value::test_record(record! {
                        "num" => Value::test_int(1),
                    }),
                    Value::test_record(record! {
                        "num" => Value::test_int(2),
                    }),
                    Value::test_record(record! {
                        "num" => Value::test_int(3),
                    }),
                ])),
            },
        ]
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_examples() {
        use super::Wrap;
        use crate::test_examples;
        test_examples(Wrap {})
    }
}
