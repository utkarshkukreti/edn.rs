use std::collections::BTreeMap;
use std::str::CharIndices;

use ordered_float::OrderedFloat;

use Value;

pub struct Parser<'a> {
    str: &'a str,
    chars: CharIndices<'a>
}

#[derive(Clone, Debug, PartialEq)]
pub struct Error {
    pub lo: usize,
    pub hi: usize,
    pub message: String
}

impl<'a> Parser<'a> {
    pub fn new(str: &'a str) -> Parser<'a> {
        Parser {
            str: str,
            chars: str.char_indices()
        }
    }

    pub fn read(&mut self) -> Option<Result<Value, Error>> {
        self.advance_while(|ch| ch.is_whitespace() || ch == ',');

        self.chars.clone().next().map(|(pos, ch)| match (pos, ch) {
            (start, '0' ... '9') => {
                let end = self.advance_while(|ch| ch.is_digit(10));
                if self.peek() == Some('.') {
                    self.chars.next();
                    let end = self.advance_while(|ch| ch.is_digit(10));
                    Ok(Value::Float(OrderedFloat(
                        self.str[start..end].parse().unwrap())))
                } else {
                    Ok(Value::Integer(self.str[start..end].parse().unwrap()))
                }
            },
            (start, ch @ '+') | (start, ch @ '-') => {
                self.chars.next();
                match self.peek() {
                    Some('0' ... '9') => {
                        let start = if ch == '+' {
                            start + 1
                        } else {
                            start
                        };
                        let end = self.advance_while(|ch| ch.is_digit(10));
                        if self.peek() == Some('.') {
                            self.chars.next();
                            let end = self.advance_while(|ch| ch.is_digit(10));
                            Ok(Value::Float(OrderedFloat(
                                self.str[start..end].parse().unwrap())))
                        } else {
                            Ok(Value::Integer(
                                self.str[start..end].parse().unwrap()))
                        }
                    },
                    Some(ch) if is_symbol_tail(ch) => {
                        let end = self.advance_while(is_symbol_tail);
                        Ok(Value::Symbol(self.str[start..end].into()))
                    },
                    None | Some(' ') | Some('\t') | Some('\n') => {
                        Ok(Value::Symbol(ch.to_string()))
                    }
                    _ => unimplemented!()
                }
            },
            (start, '\\') => {
                self.chars.next();
                let start = start + 1;
                let end = self.advance_while(|ch| !ch.is_whitespace());
                Ok(Value::Char(match &self.str[start..end] {
                    "newline" => '\n',
                    "return"  => '\r',
                    "space"   => ' ',
                    "tab"     => '\t',
                    otherwise => {
                        if otherwise.chars().count() == 1 {
                            otherwise.chars().next().unwrap()
                        } else {
                            return Err(Error {
                                lo: start - 1,
                                hi: end,
                                message: format!("invalid char literal `\\{}`",
                                                 otherwise)
                            })
                        }
                    }
                }))
            },
            (start, '"') => {
                self.chars.next();
                let mut string = String::new();
                loop {
                    match self.chars.next() {
                        Some((_, '"')) => return Ok(Value::String(string)),
                        Some((_, '\\')) => {
                            string.push(match self.chars.next() {
                                Some((_, 't'))  => '\t',
                                Some((_, 'r'))  => '\r',
                                Some((_, 'n'))  => '\n',
                                Some((_, '\\')) => '\\',
                                Some((_, '"'))  => '\"',
                                Some((pos, ch)) => return Err(Error {
                                    lo: pos - 1,
                                    hi: pos + 1,
                                    message: format!(
                                        "invalid string escape `\\{}`", ch)
                                }),
                                None       => unimplemented!()
                            });
                        },
                        Some((_, ch)) => string.push(ch),
                        None => return Err(Error {
                            lo: start,
                            hi: self.str.len(),
                            message: "expected closing `\"`, found EOF".into()
                        })
                    }
                }
            },
            (start, ':') => {
                self.chars.next();
                let end = self.advance_while(is_symbol_tail);
                Ok(Value::Keyword(self.str[start + 1..end].into()))
            },
            (start, open @ '(') | (start, open @ '[') | (start, open @ '{') => {
                let close = match open {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    _   => unreachable!()
                };

                self.chars.next();
                let mut items = vec![];
                loop {
                    self.advance_while(|ch| ch.is_whitespace() || ch == ',');

                    if self.peek() == Some(close) {
                        self.chars.next();
                        return Ok(match open {
                            '(' => Value::List(items),
                            '[' => Value::Vector(items),
                            '{' => {
                                let mut map = BTreeMap::new();
                                let mut iter = items.into_iter();
                                while let Some(key) = iter.next() {
                                    if let Some(value) = iter.next() {
                                        map.insert(key, value);
                                    } else {
                                        let end = self.chars.clone()
                                            .next()
                                            .map(|(pos, _)| pos)
                                            .unwrap_or(self.str.len());
                                        return Err(Error {
                                            lo: start,
                                            hi: end,
                                            message:
                                            "odd number of items in a Map".into()
                                        })
                                    }
                                }
                                Value::Map(map)
                            }
                            _   => unreachable!()
                        })
                    }

                    match self.read() {
                        Some(Ok(value)) => items.push(value),
                        Some(Err(err))  => return Err(err),
                        None => return Err(Error {
                            lo: start,
                            hi: self.str.len(),
                            message: format!("unclosed `{}`", open)
                        })
                    }
                }
            },
            (start, '#') => {
                self.chars.next();
                match self.chars.next() {
                    Some((_, open @ '{')) => {
                        let close = '}';
                        let mut items = vec![];
                        loop {
                            self.advance_while(|ch| ch.is_whitespace() ||
                                                    ch == ',');
                            if self.peek() == Some(close) {
                                self.chars.next();
                                return Ok(Value::Set(
                                    items.into_iter().collect()));
                            }

                            match self.read() {
                                Some(Ok(value)) => items.push(value),
                                Some(Err(err))  => return Err(err),
                                None => return Err(Error {
                                    lo: start,
                                    hi: self.str.len(),
                                    message: format!("unclosed `#{}`", open)
                                })
                            }
                        }
                    },
                    _ => unimplemented!()
                }
            }
            (start, ch) if is_symbol_head(ch) => {
                self.chars.next();
                let end = self.advance_while(is_symbol_tail);
                Ok(match &self.str[start..end] {
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
        self.chars.clone().next().map(|(_, ch)| ch)
    }

    fn advance_while<F: FnMut(char) -> bool>(&mut self, mut f: F) -> usize {
        loop {
            match self.chars.clone().next() {
                Some((pos, ch)) => {
                    if f(ch) {
                        self.chars.next();
                    } else {
                        return pos
                    }
                },
                None => return self.str.len()
            }
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
fn test_read_floats() {
    use ordered_float::OrderedFloat;

    let mut parser = Parser::new("0. 0.0 -0.0 +0.0 1.23 +1.23 -1.23");
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(1.23)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(1.23)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(-1.23)))));
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

    let mut parser = Parser::new("  \\foo  ");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 2,
        hi: 6,
        message: "invalid char literal `\\foo`".into()})));
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

    let mut parser = Parser::new("\"foo\\x\"");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 4,
        hi: 6,
        message: "invalid string escape `\\x`".into()})));

    let mut parser = Parser::new("   \"foo");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 3,
        hi: 7,
        message: "expected closing `\"`, found EOF".into()})));
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

#[test]
fn test_read_lists() {
    let mut parser = Parser::new("() (1 2 3) (true, false, nil)
                                  (((\"foo\" \"bar\")))");

    assert_eq!(parser.read(), Some(Ok(Value::List(vec![]))));

    assert_eq!(parser.read(), Some(Ok(Value::List(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3)]))));

    assert_eq!(parser.read(), Some(Ok(Value::List(vec![
        Value::Boolean(true),
        Value::Boolean(false),
        Value::Nil]))));

    assert_eq!(parser.read(), Some(Ok(
        Value::List(vec![
            Value::List(vec![
                Value::List(vec![
                    Value::String("foo".into()),
                    Value::String("bar".into())])])]))));

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("((  \\foo ))");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 4,
        hi: 8,
        message: "invalid char literal `\\foo`".into()})));

    let mut parser = Parser::new("( (  1 2 3");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 2,
        hi: 10,
        message: "unclosed `(`".into()})));
}

#[test]
fn test_read_vectors() {
    let mut parser = Parser::new("[] [1 2 3] [true, false, nil]
                                  [[[\"foo\" \"bar\"]]]");

    assert_eq!(parser.read(), Some(Ok(Value::Vector(vec![]))));

    assert_eq!(parser.read(), Some(Ok(Value::Vector(vec![
        Value::Integer(1),
        Value::Integer(2),
        Value::Integer(3)]))));

    assert_eq!(parser.read(), Some(Ok(Value::Vector(vec![
        Value::Boolean(true),
        Value::Boolean(false),
        Value::Nil]))));

    assert_eq!(parser.read(), Some(Ok(
        Value::Vector(vec![
            Value::Vector(vec![
                Value::Vector(vec![
                    Value::String("foo".into()),
                    Value::String("bar".into())])])]))));

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("[[  \\foo ]]");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 4,
        hi: 8,
        message: "invalid char literal `\\foo`".into()})));

    let mut parser = Parser::new("[ [  1 2 3");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 2,
        hi: 10,
        message: "unclosed `[`".into()})));
}

#[test]
fn test_read_maps() {
    use std::collections::BTreeMap;

    let mut parser = Parser::new("{} {1 2} {true, false}
                                  {{\"foo\" \"bar\"} \"baz\"}");

    assert_eq!(parser.read(), Some(Ok(Value::Map(BTreeMap::new()))));

    assert_eq!(parser.read(), Some(Ok(Value::Map({
        let mut map = BTreeMap::new();
        map.insert(Value::Integer(1), Value::Integer(2));
        map
    }))));

    assert_eq!(parser.read(), Some(Ok(Value::Map({
        let mut map = BTreeMap::new();
        map.insert(Value::Boolean(true), Value::Boolean(false));
        map
    }))));

    assert_eq!(parser.read(), Some(Ok(Value::Map({
        let mut map = BTreeMap::new();
        map.insert(Value::Map({
            let mut map = BTreeMap::new();
            map.insert(Value::String("foo".into()),
                       Value::String("bar".into()));
            map
        }), Value::String("baz".into()));
        map
    }))));

    let mut parser = Parser::new("{\\foo true}");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 1,
        hi: 5,
        message: "invalid char literal `\\foo`".into()})));

    let mut parser = Parser::new("{ { 1 2 3");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 2,
        hi: 9,
        message: "unclosed `{`".into()})));

    let mut parser = Parser::new("{1 2 3}");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 0,
        hi: 7,
        message: "odd number of items in a Map".into()})));

    let mut parser = Parser::new("{{1 2 3}}");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 1,
        hi: 8,
        message: "odd number of items in a Map".into()})));
}

#[test]
fn test_read_sets() {
    let mut parser = Parser::new("#{} #{1 2 2 3 3 3} #{true, false, nil}
                                  #{#{#{\"foo\" \"bar\"}}}");

    assert_eq!(parser.read(),
               Some(Ok(Value::Set([].iter().cloned().collect()))));

    assert_eq!(parser.read(), Some(Ok(Value::Set(
        [Value::Integer(1),
         Value::Integer(2),
         Value::Integer(3)].iter().cloned().collect()))));

    assert_eq!(parser.read(), Some(Ok(Value::Set(
        [Value::Boolean(true),
         Value::Boolean(false),
         Value::Nil].iter().cloned().collect()))));

    assert_eq!(parser.read(), Some(Ok(
        Value::Set(
            [Value::Set(
                [Value::Set(
                    [Value::String("foo".into()),
                     Value::String("bar".into())
                     ].iter().cloned().collect())
                 ].iter().cloned().collect())
             ].iter().cloned().collect()))));

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("#{#{  \\foo }}");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 6,
        hi: 10,
        message: "invalid char literal `\\foo`".into()})));

    let mut parser = Parser::new("#{ #{ 1 2 3");
    assert_eq!(parser.read(), Some(Err(Error {
        lo: 3,
        hi: 11,
        message: "unclosed `#{`".into()})));
}
