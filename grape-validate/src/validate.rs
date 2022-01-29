use crate::{
    ExecutableDefinitionsRule, LoneAnonymousOperationRule, OperationNameUniquenessRule,
    SingleRootField,
};
use grape_ast::{
    Context, Document, ExecutableDocument, TypeSystemDocument, TypeSystemExtensionDocument, Visitor,
};

macro_rules! validate {
    ($visit:ident($context:ident, $document:ident)) => {
        ExecutableDefinitionsRule.$visit($context, &$document);
        LoneAnonymousOperationRule.$visit($context, &$document);
        OperationNameUniquenessRule::new().$visit($context, &$document);
        SingleRootField::new().$visit($context, &$document);
    };
}

pub fn validate_document<C: Context>(context: &mut C, document: &Document) {
    validate!(visit_document(context, document));
}

pub fn validate_type_system_document<C: Context>(
    context: &mut C,
    type_system_document: &TypeSystemDocument,
) {
    validate!(visit_type_system_document(context, type_system_document));
}

pub fn validate_type_system_extension_document<C: Context>(
    context: &mut C,
    type_system_extension_document: &TypeSystemExtensionDocument,
) {
    validate!(visit_type_system_extension_document(
        context,
        type_system_extension_document
    ));
}

pub fn validate_executable_document<C: Context>(
    context: &mut C,
    executable_document: &ExecutableDocument,
) {
    validate!(visit_executable_document(context, executable_document));
}
