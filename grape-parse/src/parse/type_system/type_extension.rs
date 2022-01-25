use crate::{Error, Parse};
use grape_ast::TypeExtension;

impl<'parse> Parse<'parse> {
    pub fn type_extension(&mut self) -> Result<Option<TypeExtension>, Error> {
        std::todo!();
    }
}
