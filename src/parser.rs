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
            '+' | '-' => {
                self.advance();
                if let Some('0' ... '9') = self.peek() {
                    let start = if ch == '+' { self.pos } else { self.pos - 1 };
                    self.advance_while(|ch| '0' <= ch && ch <= '9');
                    Ok(Value::Integer(
                        self.str[start..self.pos].parse().unwrap()))
                } else {
                    unimplemented!()
                }
            },
            '\\' => {
                self.advance();
                let start = self.pos;
                self.advance_while(|ch| !ch.is_whitespace());
                Ok(Value::Char(match &self.str[start..self.pos] {
                    "newline" => '\n',
                    "return"  => '\r',
                    "space"   => ' ',
                    "tab"     => '\t',
                    otherwise => {
                        if otherwise.chars().count() == 1 {
                            otherwise.char_at(0)
                        } else {
                            unimplemented!()
                        }
                    }
                }))
            }
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

    fn advance(&mut self) {
        if self.pos < self.str.len() {
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
    let mut parser = Parser::new("0 -0 +0 +1234 1234 -1234 +9223372036854775807
                                  -9223372036854775808");
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(-1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(9223372036854775807))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(-9223372036854775808))));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_chars() {
    let mut parser = Parser::new("\\a \\π \\newline \\return \\space \\tab");
    assert_eq!(parser.read(), Some(Ok(Value::Char('a'))));
    assert_eq!(parser.read(), Some(Ok(Value::Char('π'))));
    assert_eq!(parser.read(), Some(Ok(Value::Char('\n'))));
    assert_eq!(parser.read(), Some(Ok(Value::Char('\r'))));
    assert_eq!(parser.read(), Some(Ok(Value::Char(' '))));
    assert_eq!(parser.read(), Some(Ok(Value::Char('\t'))));
    assert_eq!(parser.read(), None);
}
