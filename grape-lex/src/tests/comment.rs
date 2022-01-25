use crate::Lex;

#[test]
fn comment() {
    macro_rules! assert_comments {
        ($($source:expr,)+) => {
            $(
                let mut lex = Lex::new($source);

                assert_eq!(lex.next(), None);
            )+
        };
    }

    assert_comments! {
        "#",
        "   #",
        "# Hello, World!",
    }
}
