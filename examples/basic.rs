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
