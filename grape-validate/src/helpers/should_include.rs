use grape_ast::{BooleanValue, DocumentContext, Selection, Value, Variable};
use grape_symbol::{builtin_directive, IF};

pub fn should_include<C: DocumentContext>(context: &C, selection: &Selection) -> bool {
    let directives = selection.directives();
    let mut include = true;
    for directive in directives {
        match directive.name.symbol {
            builtin_directive::SKIP => {
                if let Some(argument) = directive.argument(&IF) {
                    match argument.value {
                        Value::Boolean(BooleanValue { span: _, value }) => include = !value,
                        Value::Variable(Variable { span: _, ref name }) => {
                            if let Some(Value::Boolean(BooleanValue { span: _, value })) =
                                context.variable(name.symbol)
                            {
                                include = !value;
                            }
                        }
                        _ => {
                            //
                        }
                    }
                }
            }
            builtin_directive::INCLUDE => {
                if let Some(argument) = directive.argument(&IF) {
                    match argument.value {
                        Value::Boolean(BooleanValue { span: _, value }) => include = value,
                        Value::Variable(Variable { span: _, ref name }) => {
                            if let Some(Value::Boolean(BooleanValue { span: _, value })) =
                                context.variable(name.symbol)
                            {
                                include = *value;
                            }
                        }
                        _ => {
                            //
                        }
                    }
                }
            }
            _ => {
                //
            }
        }
    }
    include
}
