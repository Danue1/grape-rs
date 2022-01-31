use grape_ast::{
    DirectiveDefinition, DocumentContext, DocumentVisitor, EnumTypeDefinition, EnumTypeExtension,
    InputObjectTypeDefinition, InputObjectTypeExtension, InterfaceTypeDefinition,
    InterfaceTypeExtension, ObjectTypeDefinition, ObjectTypeExtension, ScalarTypeDefinition,
    ScalarTypeExtension, SchemaDefinition, SchemaExtension, UnionTypeDefinition,
    UnionTypeExtension,
};
use grape_diagnostics::MessageBuilder;
use grape_span::Span;

/// [Spec](https://spec.graphql.org/draft/#sec-Executable-Definitions)
pub struct ExecutableDefinitionsRule;

impl<'rule, C: DocumentContext> DocumentVisitor<'rule, C> for ExecutableDefinitionsRule {
    fn visit_schema_definition(
        &mut self,
        context: &C,
        _schema_definition: &'rule SchemaDefinition,
    ) {
        report(context, ReportKind::DefinitionName(SchemaDefinition::NAME));
    }

    fn visit_scalar_type_definition(
        &mut self,
        context: &C,
        scalar_type_definition: &'rule ScalarTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&scalar_type_definition.name.span),
        );
    }

    fn visit_object_type_definition(
        &mut self,
        context: &C,
        object_type_definition: &'rule ObjectTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&object_type_definition.name.span),
        );
    }

    fn visit_interface_type_definition(
        &mut self,
        context: &C,
        interface_type_definition: &'rule InterfaceTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&interface_type_definition.name.span),
        );
    }

    fn visit_union_type_definition(
        &mut self,
        context: &C,
        union_type_definition: &'rule UnionTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&union_type_definition.name.span),
        );
    }

    fn visit_enum_type_definition(
        &mut self,
        context: &C,
        enum_type_definition: &'rule EnumTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&enum_type_definition.name.span),
        );
    }

    fn visit_input_object_type_definition(
        &mut self,
        context: &C,
        input_object_type_definition: &'rule InputObjectTypeDefinition,
    ) {
        report(
            context,
            ReportKind::DefinitionSpan(&input_object_type_definition.name.span),
        );
    }

    fn visit_directive_definition(
        &mut self,
        context: &C,
        directive_definition: &'rule DirectiveDefinition,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&directive_definition.name.span),
        );
    }

    fn visit_schema_extension(&mut self, context: &C, _schema_extension: &'rule SchemaExtension) {
        report(context, ReportKind::ExtensionName(SchemaExtension::NAME));
    }

    fn visit_scalar_type_extension(
        &mut self,
        context: &C,
        scalar_type_extension: &'rule ScalarTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&scalar_type_extension.name.span),
        );
    }

    fn visit_object_type_extension(
        &mut self,
        context: &C,
        object_type_extension: &'rule ObjectTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&object_type_extension.name.span),
        );
    }

    fn visit_interface_type_extension(
        &mut self,
        context: &C,
        interface_type_extension: &'rule InterfaceTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&interface_type_extension.name.span),
        );
    }

    fn visit_union_type_extension(
        &mut self,
        context: &C,
        union_type_extension: &'rule UnionTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&union_type_extension.name.span),
        );
    }

    fn visit_enum_type_extension(
        &mut self,
        context: &C,
        enum_type_extension: &'rule EnumTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&enum_type_extension.name.span),
        );
    }

    fn visit_input_object_type_extension(
        &mut self,
        context: &C,
        input_object_type_extension: &'rule InputObjectTypeExtension,
    ) {
        report(
            context,
            ReportKind::ExtensionSpan(&input_object_type_extension.name.span),
        );
    }
}

enum ReportKind<'report> {
    DefinitionSpan(&'report Span),
    DefinitionName(&'report str),
    ExtensionSpan(&'report Span),
    ExtensionName(&'report str),
}

fn report<C: DocumentContext>(context: &C, kind: ReportKind) {
    let message = match kind {
        ReportKind::DefinitionSpan(span) => {
            format!(
                "The \"{}\" definition is not executable.",
                context.span(*span)
            )
        }
        ReportKind::DefinitionName(name) => {
            format!("The {} definition is not executable.", name)
        }
        ReportKind::ExtensionSpan(span) => {
            format!(
                "The \"{}\" extension is not executable.",
                context.span(*span)
            )
        }
        ReportKind::ExtensionName(name) => {
            format!("The {} extension is not executable.", name)
        }
    };

    context.report(MessageBuilder::error(&message).build());
}
