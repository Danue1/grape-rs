use crate::assert_document;

#[test]
fn without_fragment() {
    assert_document!(
        "
            subscription sub {
                newMessage {
                    body
                    sender
                }
                disallowedSecondRootField
            }
        ",
        "error: Subscription operations must have exactly one root field.\n\
        Multiple fields with the name \"disallowedSecondRootField\" are not allowed in a single root field."
    );
}

#[test]
fn with_fragment() {
    assert_document!(
        "
            subscription sub {
                ...multipleSubscriptions
            }

            fragment multipleSubscriptions on Subscription {
                newMessage {
                    body
                    sender
                }
                disallowedSecondRootField
            }
        ",
        "error: Subscription operations must have exactly one root field.\n\
        Multiple fields with the name \"disallowedSecondRootField\" are not allowed in a single root field."
    );
}

#[test]
fn deny_introspection() {
    assert_document!(
        "
        subscription sub {
            __typename
        }
        ",
        "error: \"__typename\" is not allowed in a single root field."
    );
}
