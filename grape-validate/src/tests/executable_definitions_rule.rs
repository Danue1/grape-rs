use crate::assert_document;

#[test]
fn executable_definitions_rule() {
    assert_document!(
        "
            query getDogName {
                dog {
                    name
                    color
                }
            }

            extend type Dog {
                color: String
            }
        ",
        "error: The \"Dog\" extension is not executable."
    );
}
