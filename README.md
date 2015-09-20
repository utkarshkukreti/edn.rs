# edn.rs

> An EDN (Extensible Data Notation) parser.

## Examples

```rust
extern crate edn;

use edn::parser::Parser;

fn main() {
    let str = "(defn sum [xs]
                 (reduce + 0 xs))
               (println (sum [1 2 3 4 5]))";

    let mut parser = Parser::new(str);
    println!("{:?}", parser.read());
    println!("{:?}", parser.read());
}
```

prints

```rust
Some(Ok(List([Symbol("defn"), Symbol("sum"), Vector([Symbol("xs")]), List([Symbol("reduce"), Symbol("+"), Integer(0), Symbol("xs")])])))
Some(Ok(List([Symbol("println"), List([Symbol("sum"), Vector([Integer(1), Integer(2), Integer(3), Integer(4), Integer(5)])])])))
```

## License

MIT
