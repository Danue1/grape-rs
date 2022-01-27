use crate::{error, Parse};
use grape_ast::{
    DirectiveLocation, ExecutableDirectiveLocation, ExecutableDirectiveLocationKind,
    TypeSystemDirectiveLocation, TypeSystemDirectiveLocationKind,
};
use grape_diagnostics::Message;
use grape_symbol::{executable, type_system};
use grape_token::TokenKind;

impl<'parse> Parse<'parse> {
    pub fn directive_locations(&mut self) -> Result<Vec<DirectiveLocation>, Message> {
        if self.current_token() == &TokenKind::Pipe {
            self.bump();
        }

        let mut locations = vec![self.directive_location()?];

        while self.current_token() == &TokenKind::Pipe {
            self.bump();

            locations.push(self.directive_location()?);
        }

        Ok(locations)
    }

    pub fn directive_location(&mut self) -> Result<DirectiveLocation, Message> {
        macro_rules! match_location {
            (
                executable: {
                    $($executable_token:ident : $executable_kind:ident,)+
                },
                type_system: {
                    $($type_system_token:ident : $type_system_kind:ident,)+
                },
            ) => {
                match self.current() {
                    $(
                        (span, TokenKind::Name(executable::$executable_token)) => {
                            let location = ExecutableDirectiveLocation {
                                span: *span,
                                kind: ExecutableDirectiveLocationKind::$executable_kind
                            };

                            self.bump();

                            Ok(DirectiveLocation::Executable(location))
                        }
                    )+
                    $(
                        (span, TokenKind::Name(type_system::$type_system_token)) => {
                            let location = TypeSystemDirectiveLocation {
                                span: *span,
                                kind: TypeSystemDirectiveLocationKind::$type_system_kind
                            };

                            self.bump();

                            Ok(DirectiveLocation::TypeSystem(location))
                        }
                    )+
                    _ => error!()
                }
            };
        }

        match_location! {
            executable: {
                QUERY: Query,
                MUTATION: Mutation,
                SUBSCRIPTION: Subscription,
                FIELD: Field,
                FRAGMENT_DEFINITION: FragmentDefinition,
                FRAGMENT_SPREAD: FragmentSpread,
                INLINE_FRAGMENT: InlineFragment,
                VARIABLE_DEFINITION: VariableDefinition,
            },
            type_system: {
                SCHEMA: Schema,
                SCALAR: Scalar,
                OBJECT: Object,
                FIELD_DEFINITION: FieldDefinition,
                ARGUMENT_DEFINITION: ArgumentDefinition,
                INTERFACE: Interface,
                UNION: Union,
                ENUM: Enum,
                ENUM_VALUE: EnumValue,
                INPUT_OBJECT: InputObject,
                INPUT_FIELD_DEFINITION: InputFieldDefinition,
            },
        }
    }
}
