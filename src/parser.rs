use Value;

pub struct Parser<'a> {
    str: &'a str,
    pos: usize
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Parser<'a> {
        Parser {
            str: str,
            pos: 0
        }
    }

    pub fn read(&mut self) -> Option<Result<Value, ()>> {
        None
    }
}

#[test]
fn test_read_empty() {
    let mut parser = Parser::new("");
    assert_eq!(parser.read(), None);
}
