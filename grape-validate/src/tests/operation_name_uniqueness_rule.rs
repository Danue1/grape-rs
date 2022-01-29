use crate::assert_document;

#[test]
fn operation_name_uniqueness_rule() {
    assert_document!(
        "
            query getName {
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
        "error: There can be only one operation named \"getName\"."
    );
}
