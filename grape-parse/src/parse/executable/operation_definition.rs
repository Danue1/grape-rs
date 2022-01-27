use crate::{error, Parse};
use grape_ast::{OperationDefinition, OperationType};
use grape_diagnostics::Message;

impl<'parse> Parse<'parse> {
    pub fn operation_definition(&mut self) -> Result<Option<OperationDefinition>, Message> {
        let start_span = self.span;
        if let Ok((_, operation_type)) = self.operation_type() {
            let name = self.name().ok();
            let variables = self.variable_definitions()?;
            let directives = self.directives()?;

            if let Some((span, selections)) = self.selections()? {
                Ok(Some(OperationDefinition {
                    span: start_span.with_end(&span),
                    operation_type,
                    name,
                    variables,
                    directives,
                    selections,
                }))
            } else {
                error!()
            }
        } else if let Some((span, selections)) = self.selections()? {
            Ok(Some(OperationDefinition {
                span,
                operation_type: OperationType::Query,
                name: None,
                variables: vec![],
                directives: vec![],
                selections,
            }))
        } else {
            Ok(None)
        }
    }
}
