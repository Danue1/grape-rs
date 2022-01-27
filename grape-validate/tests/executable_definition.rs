use grape_parse::Parse;
use grape_validate::{validate_document, ValidationContext};

#[test]
fn executable_definition() {
    let source = "
        query getDogName {
            dog {
                name
                color
            }
        }

        extend type Dog {
            color: String
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
        "error: The \"Dog\" extension is not executable."
    );
}
