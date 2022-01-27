use grape_parse::Parse;
use grape_validate::{validate_document, ValidationContext};

#[test]
fn lone_anonymous_operation() {
    let source = "
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
    ";
    let document = {
        let document = Parse::new(source).document();
        assert_eq!(document.is_ok(), true);
        document.unwrap()
    };

    let mut context = ValidationContext::new(source);
    validate_document(&mut context, &document);

    assert_eq!(
        context.diagnostics.to_string(),
        "error: There can be only one operation named \"getName\"."
    );
}
