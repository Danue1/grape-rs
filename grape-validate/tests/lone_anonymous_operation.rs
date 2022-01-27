use grape_parse::Parse;
use grape_validate::{validate_document, ValidationContext};

#[test]
fn lone_anonymous_operation() {
    let source = "
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
        "error: This anonymous operation must be the only defined operation."
    );
}
