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
            (start, '.') => {
                self.chars.next();
                if let Some('0' ... '9') = self.peek() {
                    let end = self.advance_while(|ch| ch.is_digit(10));
                    Ok(Value::Float(OrderedFloat(
                        self.str[start..end].parse().unwrap())))
                } else {
                    let end = self.advance_while(is_symbol_tail);
                    Ok(Value::Symbol(self.str[start..end].into()))
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
