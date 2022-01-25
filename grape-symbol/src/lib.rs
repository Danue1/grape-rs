#![warn(clippy::all)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Symbol(SymbolIndex);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct SymbolIndex {
    index: usize,
}

#[derive(Debug)]
pub struct SymbolInterner<'interner> {
    pub(crate) strings: Vec<&'interner str>,
    pub(crate) symbols: HashMap<&'interner str, Symbol>,
}

impl Symbol {
    pub const fn is_empty(self) -> bool {
        matches!(self, EMPTY)
    }

    pub const fn is_boolean(self) -> bool {
        matches!(self, TRUE | FALSE)
    }

    pub const fn is_null(self) -> bool {
        matches!(self, NULL)
    }

    pub const fn is_executable(self) -> bool {
        use executable::*;

        matches!(
            self,
            QUERY
                | MUTATION
                | SUBSCRIPTION
                | FIELD
                | FRAGMENT_DEFINITION
                | FRAGMENT_SPREAD
                | INLINE_FRAGMENT
                | VARIABLE_DEFINITION
        )
    }

    pub const fn is_operation_type(self) -> bool {
        matches!(self, QUERY | MUTATION | SUBSCRIPTION)
    }

    pub const fn is_type_system(self) -> bool {
        use type_system::*;

        matches!(
            self,
            SCHEMA
                | SCALAR
                | OBJECT
                | FIELD_DEFINITION
                | ARGUMENT_DEFINITION
                | INTERFACE
                | UNION
                | ENUM
                | ENUM_VALUE
                | INPUT_OBJECT
                | INPUT_FIELD_DEFINITION
        )
    }

    pub const fn is_builtin_directive(self) -> bool {
        use builtin_directive::*;

        matches!(self, SKIP | INCLUDE | DEPRECATED | SPECIFIED_BY)
    }
}

impl<'interner> SymbolInterner<'interner> {
    #[inline]
    pub fn new() -> Self {
        SymbolInterner::default()
    }

    pub fn intern(&mut self, string: &'interner str) -> Symbol {
        if let Some(&symbol) = self.symbols.get(string) {
            symbol
        } else {
            let symbol = Symbol(SymbolIndex {
                index: self.strings.len(),
            });

            self.strings.push(string);
            self.symbols.insert(string, symbol);

            symbol
        }
    }
}

macro_rules! impl_default_for_symbol_interner {
    (
        pub mod inline {
            $(
                $(#[$inline_meta:meta])*
                pub const $inline_key:ident : Symbol = $inline_value:expr;
            )+
        }
        pub mod executable {
            $(
                pub const $executable_key:ident : Symbol = $executable_value:expr;
            )+
        }
        pub mod type_system {
            $(
                pub const $type_system_key:ident : Symbol = $type_system_value:expr;
            )+
        }
        pub mod builtin_directive {
            $(
                pub const $builtin_directive_key:ident : Symbol = $builtin_directive_value:expr;
            )+
        }
    ) => {
        const_symbol!(@step 0;
            $(
                #[$($inline_meta)*]
                $inline_key;
            )+
        );

        pub mod executable {
            use crate::{Symbol, SymbolIndex};

            const_symbol!(
                @step count_symbol!(@step 0; $($inline_key)+);
                $(
                    /// [Spec](https://spec.graphql.org/October2021/#ExecutableDirectiveLocation)
                    $executable_key;
                )+
            );
        }

        pub mod type_system {
            use crate::{Symbol, SymbolIndex};

            const_symbol!(
                @step count_symbol!(@step 0; $($inline_key)+ $($executable_key)+);
                $(
                    /// [Spec](https://spec.graphql.org/October2021/#TypeSystemDirectiveLocation)
                    $type_system_key;
                )+
            );
        }

        pub mod builtin_directive {
            use crate::{Symbol, SymbolIndex};

            const_symbol!(
                @step count_symbol!(@step 0; $($inline_key)+ $($executable_key)+ $($type_system_key)+);
                $(
                    /// [Spec](https://spec.graphql.org/October2021/#sec-Type-System.Directives.Built-in-Directives)
                    $builtin_directive_key;
                )+
            );
        }

        impl<'interner> Default for SymbolInterner<'interner> {
            fn default() -> Self {
                SymbolInterner {
                    strings: vec![$($inline_value,)+ $($executable_value,)+ $($type_system_value,)+ $($builtin_directive_value,)+],
                    symbols: {
                        let mut hash_map = HashMap::new();
                        $(hash_map.insert($inline_value, $inline_key);)+
                        $(hash_map.insert($executable_value, executable::$executable_key);)+
                        $(hash_map.insert($type_system_value, type_system::$type_system_key);)+
                        $(hash_map.insert($builtin_directive_value, builtin_directive::$builtin_directive_key);)+
                        hash_map
                    },
                }
            }
        }
    };
}

macro_rules! const_symbol {
    (@step $_index:expr;) => {
        //
    };

    (@step $index:expr; #[$($meta:meta)*] $symbol:ident; $(#[$($tail_meta:meta)*] $tail_symbol:ident;)*) => {
        $(#[$meta])*
        pub const $symbol: Symbol = Symbol(SymbolIndex { index: $index });
        const_symbol!(@step $index + 1; $(#[$($tail_meta)*] $tail_symbol;)*);
    }
}

macro_rules! count_symbol {
    (@step $index:expr;) => {
        $index
    };

    (@step $index:expr; $symbol:ident $($tail_symbol:ident)*) => {
        count_symbol!(@step $index + 1; $($tail_symbol)*)
    }
}

impl_default_for_symbol_interner! {
    pub mod inline {
        pub const EMPTY: Symbol = "";
        pub const EMPTY_STRING: Symbol = "\"\"";
        pub const BOOLEAN: Symbol = "Boolean";
        pub const INT: Symbol = "Int";
        pub const FLOAT: Symbol = "Float";
        pub const STRING: Symbol = "String";

        /// [Spec](https://spec.graphql.org/October2021/#BooleanValue)
        pub const TRUE: Symbol = "true";

        /// [Spec](https://spec.graphql.org/October2021/#BooleanValue)
        pub const FALSE: Symbol = "false";

        /// [Spec](https://spec.graphql.org/October2021/#NullValue)
        pub const NULL: Symbol = "null";

        /// [Spec](https://spec.graphql.org/October2021/#OperationType)
        pub const QUERY: Symbol = "query";

        /// [Spec](https://spec.graphql.org/October2021/#OperationType)
        pub const MUTATION: Symbol = "mutation";

        /// [Spec](https://spec.graphql.org/October2021/#OperationType)
        pub const SUBSCRIPTION: Symbol = "subscription";

        /// [Spec](https://spec.graphql.org/October2021/#TypeCondition)
        pub const ON: Symbol = "on";

        /// [Spec](https://spec.graphql.org/October2021/#FragmentDefinition)
        pub const FRAGMENT: Symbol = "fragment";

        /// [Spec](https://spec.graphql.org/October2021/#SchemaDefinition)
        pub const SCHEMA: Symbol = "schema";

        /// [Spec::Schema](https://spec.graphql.org/October2021/#SchemaExtension)
        /// [Spec::Scalar](https://spec.graphql.org/October2021/#ScalarTypeExtension)
        /// [Spec::Type](https://spec.graphql.org/October2021/#ObjectTypeExtension)
        /// [Spec::Interface](https://spec.graphql.org/October2021/#InterfaceTypeExtension)
        /// [Spec::Union](https://spec.graphql.org/October2021/#UnionTypeExtension)
        /// [Spec::Enum](https://spec.graphql.org/October2021/#EnumTypeExtension)
        /// [Spec::Input](https://spec.graphql.org/October2021/#InputObjectTypeExtension)
        pub const EXTEND: Symbol = "extend";

        /// [Spec](https://spec.graphql.org/October2021/#ScalarTypeDefinition)
        pub const SCALAR: Symbol = "scalar";

        /// [Spec](https://spec.graphql.org/October2021/#ObjectTypeDefinition)
        pub const TYPE: Symbol = "type";

        /// [Spec](https://spec.graphql.org/October2021/#ImplementsInterfaces)
        pub const IMPLEMENTS: Symbol = "implements";

        /// [Spec](https://spec.graphql.org/October2021/#InterfaceTypeDefinition)
        pub const INTERFACE: Symbol = "interface";

        /// [Spec](https://spec.graphql.org/October2021/#UnionTypeDefinition)
        pub const UNION: Symbol = "union";

        /// [Spec](https://spec.graphql.org/October2021/#EnumTypeDefinition)
        pub const ENUM: Symbol = "enum";

        /// [Spec](https://spec.graphql.org/October2021/#InputObjectTypeDefinition)
        pub const INPUT: Symbol = "input";

        /// [Spec](https://spec.graphql.org/October2021/#DirectiveDefinition)
        pub const DIRECTIVE: Symbol = "directive";

        /// [Spec](https://spec.graphql.org/October2021/#DirectiveDefinition)
        pub const REPEATABLE: Symbol = "repeatable";
    }

    pub mod executable {
        pub const QUERY: Symbol = "QUERY";
        pub const MUTATION: Symbol = "MUTATION";
        pub const SUBSCRIPTION: Symbol = "SUBSCRIPTION";
        pub const FIELD: Symbol = "FIELD";
        pub const FRAGMENT_DEFINITION: Symbol = "FRAGMENT_DEFINITION";
        pub const FRAGMENT_SPREAD: Symbol = "FRAGMENT_SPREAD";
        pub const INLINE_FRAGMENT: Symbol = "INLINE_FRAGMENT";
        pub const VARIABLE_DEFINITION: Symbol = "VARIABLE_DEFINITION";
    }

    pub mod type_system {
        pub const SCHEMA: Symbol = "SCHEMA";
        pub const SCALAR: Symbol = "SCALAR";
        pub const OBJECT: Symbol = "OBJECT";
        pub const FIELD_DEFINITION: Symbol = "FIELD_DEFINITION";
        pub const ARGUMENT_DEFINITION: Symbol = "ARGUMENT_DEFINITION";
        pub const INTERFACE: Symbol = "INTERFACE";
        pub const UNION: Symbol = "UNION";
        pub const ENUM: Symbol = "ENUM";
        pub const ENUM_VALUE: Symbol = "ENUM_VALUE";
        pub const INPUT_OBJECT: Symbol = "INPUT_OBJECT";
        pub const INPUT_FIELD_DEFINITION: Symbol = "INPUT_FIELD_DEFINITION";
    }

    pub mod builtin_directive {
        pub const SKIP: Symbol = "skip";
        pub const INCLUDE: Symbol = "include";
        pub const DEPRECATED: Symbol = "deprecated";
        pub const SPECIFIED_BY: Symbol = "specifiedby";
    }
}
