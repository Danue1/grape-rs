use crate::assert_document;

#[test]
fn lone_anonymous_operation_rule() {
    assert_document!(
        "
            {
                dog {
                    name
                }
            }

            query getName {
                dog {
                    owner {
                        name
                    }
                }
            }
        ",
        "error: This anonymous operation must be the only defined operation."
    );
}
