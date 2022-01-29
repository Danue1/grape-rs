#[macro_export]
macro_rules! assert_document {
    ($source:expr, $error:expr) => {{
        use std::collections::HashMap;

        let document = {
            let document = grape_parse::Parse::new($source).document();
            assert_eq!(document.is_ok(), true);
            document.unwrap()
        };

        let variables = HashMap::default();
        let mut context = $crate::ValidationContext::new($source, &document, &variables);
        $crate::validate_document(&mut context, &document);

        assert_eq!(context.diagnostics.borrow().to_string(), $error);
    }};
}
