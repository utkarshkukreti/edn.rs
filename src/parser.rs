use Value;

pub struct Parser<'a> {
    str: &'a str,
    pos: usize
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    lo: usize,
    hi: usize,
    message: String
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Parser<'a> {
        Parser {
            str: str,
            pos: 0
        }
    }

    pub fn read(&mut self) -> Option<Result<Value, Error>> {
        self.advance_while(|ch| ch.is_whitespace() || ch == ',');

        self.peek().map(|ch| match ch {
            '0' ... '9' => {
                let start = self.pos;
                self.advance_while(|ch| '0' <= ch && ch <= '9');
                Ok(Value::Integer(self.str[start..self.pos].parse().unwrap()))
            },
            '+' | '-' => {
                self.advance();
                match self.peek() {
                    Some('0' ... '9') => {
                        let start = if ch == '+' {
                            self.pos
                        } else {
                            self.pos - 1
                        };
                        self.advance_while(|ch| '0' <= ch && ch <= '9');
                        Ok(Value::Integer(
                            self.str[start..self.pos].parse().unwrap()))
                    },
                    Some(ch) if is_symbol_tail(ch) => {
                        let start = self.pos - 1;
                        self.advance_while(is_symbol_tail);
                        Ok(Value::Symbol(self.str[start..self.pos].into()))
                    },
                    None | Some(' ') | Some('\t') | Some('\n') => {
                        Ok(Value::Symbol(ch.to_string()))
                    }
                    _ => unimplemented!()
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
            },
            '"' => {
                self.advance();
                let mut string = String::new();
                loop {
                    match self.advance() {
                        Some('"') => return Ok(Value::String(string)),
                        Some('\\') => {
                            string.push(match self.advance() {
                                Some('t')  => '\t',
                                Some('r')  => '\r',
                                Some('n')  => '\n',
                                Some('\\') => '\\',
                                Some('"')  => '\"',
                                Some(_)    => unimplemented!(),
                                None       => unimplemented!()
                            });
                        },
                        Some(ch) => string.push(ch),
                        None => unimplemented!()
                    }
                }
            },
            ':' => {
                self.advance();
                let start = self.pos;
                self.advance_while(is_symbol_tail);
                Ok(Value::Keyword(self.str[start..self.pos].into()))
            }
            ch if is_symbol_head(ch) => {
                let start = self.pos;
                self.advance();
                self.advance_while(is_symbol_tail);
                Ok(match &self.str[start..self.pos] {
                    "true"    => Value::Boolean(true),
                    "false"   => Value::Boolean(false),
                    "nil"     => Value::Nil,
                    otherwise => Value::Symbol(otherwise.into())
                })
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

    fn advance(&mut self) -> Option<char> {
        if self.pos < self.str.len() {
            let range = self.str.char_range_at(self.pos);
            self.pos = range.next;
            Some(range.ch)
        } else {
            None
        }
    }
}

fn is_symbol_head(ch: char) -> bool {
    match ch {
        'a' ... 'z' | 'A' ... 'Z' |
        '.' | '*' | '+' | '!' | '-' | '_' |
        '?' | '$' | '%' | '&' | '=' | '<' | '>' => true,
        _ => false
    }
}

fn is_symbol_tail(ch: char) -> bool {
    is_symbol_head(ch) || match ch {
        '0' ... '9' | ':' | '#' => true,
        _ => false
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

#[test]
fn test_read_strings() {
    let mut parser = Parser::new(r#"
"foo"
"bar"
"baz
quux"
"\t\r\n\\\""
"#);
    assert_eq!(parser.read(), Some(Ok(Value::String("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("bar".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("baz\nquux".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("\t\r\n\\\"".into()))));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_symbols() {
    let mut parser = Parser::new(r#"
foo
+foo
-foo
.foo
.*+!-_?$%&=<>:#123
+
-
"#);
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("+foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("-foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol(".foo".into()))));
    assert_eq!(parser.read(),
               Some(Ok(Value::Symbol(".*+!-_?$%&=<>:#123".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("+".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("-".into()))));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_keywords() {
    let mut parser = Parser::new(r#"
:foo
:+foo
:-foo
:.foo
:.*+!-_?$%&=<>:#123
:+
:-
"#);
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("+foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("-foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword(".foo".into()))));
    assert_eq!(parser.read(),
               Some(Ok(Value::Keyword(".*+!-_?$%&=<>:#123".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("+".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("-".into()))));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_booleans_and_nil() {
    let mut parser = Parser::new("true false nil");
    assert_eq!(parser.read(), Some(Ok(Value::Boolean(true))));
    assert_eq!(parser.read(), Some(Ok(Value::Boolean(false))));
    assert_eq!(parser.read(), Some(Ok(Value::Nil)));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_commas() {
    let mut parser = Parser::new(",, true ,false,");
    assert_eq!(parser.read(), Some(Ok(Value::Boolean(true))));
    assert_eq!(parser.read(), Some(Ok(Value::Boolean(false))));
    assert_eq!(parser.read(), None);
}
