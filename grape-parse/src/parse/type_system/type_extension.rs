use crate::{Error, Parse};
use grape_ast::TypeExtension;
use grape_span::Span;
use grape_symbol::EXTEND;
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn type_extension(&mut self) -> Result<Option<TypeExtension>, Error> {
        if let (&start_span, TokenKind::Name(EXTEND)) = self.current() {
            self.bump();

            self.type_extension_with_extend(&start_span)
        } else {
            Ok(None)
        }
    }

    pub fn type_extension_with_extend(
        &mut self,
        start_span: &Span,
    ) -> Result<Option<TypeExtension>, Error> {
        if let Some(extension) = self.scalar_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::Scalar(extension)))
        } else if let Some(extension) = self.object_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::Object(extension)))
        } else if let Some(extension) = self.interface_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::Interface(extension)))
        } else if let Some(extension) = self.union_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::Union(extension)))
        } else if let Some(extension) = self.enum_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::Enum(extension)))
        } else if let Some(extension) = self.input_object_type_extension_with_extend(start_span)? {
            Ok(Some(TypeExtension::InputObject(extension)))
        } else {
            Ok(None)
        }
    }
}
