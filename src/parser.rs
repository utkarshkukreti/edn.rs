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
        self.advance_while(|ch| ch.is_whitespace());

        self.peek().map(|ch| match ch {
            '0' ... '9' => {
                let start = self.pos;
                self.advance_while(|ch| '0' <= ch && ch <= '9');
                Ok(Value::Integer(self.str[start..self.pos].parse().unwrap()))
            },
            _ => unimplemented!(),
        })
    }

    fn peek(&self) -> Option<char> {
        if self.pos < self.str.len() {
            Some(self.str.char_at(self.pos))
        } else {
            None
        }
    }

    fn advance_while<F: FnMut(char) -> bool>(&mut self, mut f: F) {
        while self.peek().map_or(false, |ch| f(ch)) {
            self.pos = self.str.char_range_at(self.pos).next
        }
    }
}

#[test]
fn test_read_empty() {
    let mut parser = Parser::new("");
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_integers() {
    let mut parser = Parser::new("0 1234 9223372036854775807");
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(9223372036854775807))));
    assert_eq!(parser.read(), None);
}
