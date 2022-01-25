use crate::{Error, Parse};
use grape_ast::TypeSystemExtension;

impl<'parse> Parse<'parse> {
    pub fn type_system_extension(&mut self) -> Result<Option<TypeSystemExtension>, Error> {
        if let Some(extension) = self.schema_extension()? {
            Ok(Some(TypeSystemExtension::Schema(extension)))
        } else if let Some(extension) = self.type_extension()? {
            Ok(Some(TypeSystemExtension::Type(extension)))
        } else {
            Ok(None)
        }
    }
}
