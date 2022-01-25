use crate::{Error, Parse};
use grape_ast::SchemaExtension;

impl<'parse> Parse<'parse> {
    pub fn schema_extension(&mut self) -> Result<Option<SchemaExtension>, Error> {
        std::todo!();
    }
}
