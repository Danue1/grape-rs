# Grape

`Grape` is a GraphQL Lexer, Parser, Validator, and Runtime.

```rust
use grape::Parse;

fn main() {
  let source = "{ post { user } }";
  let mut parse = Parse::new(source);
  let document = parse.document();
  println!("{:#?}", document);
}
```
