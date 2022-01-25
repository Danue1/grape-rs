use crate::Parse;
use grape_ast::{
    DirectiveLocation, ExecutableDirectiveLocation, ExecutableDirectiveLocationKind,
    TypeSystemDirectiveLocation, TypeSystemDirectiveLocationKind,
};
use grape_span::Span;

#[test]
fn executable() {
    macro_rules! assert_executable {
        (
            without_pipe {
                $($without_pipe_source:expr => $without_pipe_kind:ident,)+
            }
            with_pipe {
                $($with_pipe_source:expr => $with_pipe_kind:ident,)+
            }
        ) => {
            $(
                let mut parse = Parse::new($without_pipe_source);

                assert_eq!(
                    parse.directive_locations(),
                    Ok(vec![DirectiveLocation::Executable(
                        ExecutableDirectiveLocation {
                            span: Span::new(0, $without_pipe_source.len()),
                            kind: ExecutableDirectiveLocationKind::$without_pipe_kind,
                        },
                    )]),
                );
            )+
            $(
                let mut parse = Parse::new($with_pipe_source);

                assert_eq!(
                    parse.directive_locations(),
                    Ok(vec![DirectiveLocation::Executable(
                        ExecutableDirectiveLocation {
                            span: Span::new(2, $with_pipe_source.len()),
                            kind: ExecutableDirectiveLocationKind::$with_pipe_kind,
                        },
                    )]),
                );
            )+
        };
    }

    assert_executable! {
        without_pipe {
            "QUERY" => Query,
            "MUTATION" => Mutation,
            "SUBSCRIPTION" => Subscription,
            "FIELD" => Field,
            "FRAGMENT_DEFINITION" => FragmentDefinition,
            "FRAGMENT_SPREAD" => FragmentSpread,
            "INLINE_FRAGMENT" => InlineFragment,
            "VARIABLE_DEFINITION" => VariableDefinition,
        }
        with_pipe {
            "| QUERY" => Query,
            "| MUTATION" => Mutation,
            "| SUBSCRIPTION" => Subscription,
            "| FIELD" => Field,
            "| FRAGMENT_DEFINITION" => FragmentDefinition,
            "| FRAGMENT_SPREAD" => FragmentSpread,
            "| INLINE_FRAGMENT" => InlineFragment,
            "| VARIABLE_DEFINITION" => VariableDefinition,
        }
    }
}

#[test]
fn type_system() {
    macro_rules! assert_type_system {
        (
            without_pipe {
                $($without_pipe_source:expr => $without_pipe_kind:ident,)+
            }
            with_pipe {
                $($with_pipe_source:expr => $with_pipe_kind:ident,)+
            }
        ) => {
            $(
                let mut parse = Parse::new($without_pipe_source);

                assert_eq!(
                    parse.directive_locations(),
                    Ok(vec![DirectiveLocation::TypeSystem(
                        TypeSystemDirectiveLocation {
                            span: Span::new(0, $without_pipe_source.len()),
                            kind: TypeSystemDirectiveLocationKind::$without_pipe_kind,
                        },
                    )]),
                );
            )+
            $(
                let mut parse = Parse::new($with_pipe_source);

                assert_eq!(
                    parse.directive_locations(),
                    Ok(vec![DirectiveLocation::TypeSystem(
                        TypeSystemDirectiveLocation {
                            span: Span::new(2, $with_pipe_source.len()),
                            kind: TypeSystemDirectiveLocationKind::$with_pipe_kind,
                        },
                    )]),
                );
            )+
        };
    }

    assert_type_system! {
        without_pipe {
            "SCHEMA" => Schema,
            "SCALAR" => Scalar,
            "OBJECT" => Object,
            "FIELD_DEFINITION" => FieldDefinition,
            "ARGUMENT_DEFINITION" => ArgumentDefinition,
            "INTERFACE" => Interface,
            "UNION" => Union,
            "ENUM" => Enum,
            "ENUM_VALUE" => EnumValue,
            "INPUT_OBJECT" => InputObject,
            "INPUT_FIELD_DEFINITION" => InputFieldDefinition,
        }
        with_pipe {
            "| SCHEMA" => Schema,
            "| SCALAR" => Scalar,
            "| OBJECT" => Object,
            "| FIELD_DEFINITION" => FieldDefinition,
            "| ARGUMENT_DEFINITION" => ArgumentDefinition,
            "| INTERFACE" => Interface,
            "| UNION" => Union,
            "| ENUM" => Enum,
            "| ENUM_VALUE" => EnumValue,
            "| INPUT_OBJECT" => InputObject,
            "| INPUT_FIELD_DEFINITION" => InputFieldDefinition,
        }
    }
}
