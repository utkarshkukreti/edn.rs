extern crate edn;
extern crate ordered_float;

use edn::parser::{Error, Parser};
use edn::Value;

#[test]
fn test_read_empty() {
    let mut parser = Parser::new("");
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_integers() {
    let mut parser = Parser::new(
        "0 -0 +0 +1234 1234 -1234 +9223372036854775807
                                  -9223372036854775808",
    );
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(0))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(-1234))));
    assert_eq!(parser.read(), Some(Ok(Value::Integer(9223372036854775807))));
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Integer(-9223372036854775808)))
    );
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_floats() {
    use ordered_float::OrderedFloat;

    let mut parser = Parser::new("0. 0.0 -0.0 +0.0 1.23 +1.23 -1.23 .125");
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.0)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(1.23)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(1.23)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(-1.23)))));
    assert_eq!(parser.read(), Some(Ok(Value::Float(OrderedFloat(0.125)))));
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
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 2,
            hi: 6,
            message: "invalid char literal `\\foo`".into()
        }))
    );
}

#[test]
fn test_read_strings() {
    let mut parser = Parser::new(
        r#"
"foo"
"bar"
"baz
quux"
"\t\r\n\\\""
"#,
    );
    assert_eq!(parser.read(), Some(Ok(Value::String("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("bar".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("baz\nquux".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::String("\t\r\n\\\"".into()))));
    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("\"foo\\x\"");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 4,
            hi: 6,
            message: "invalid string escape `\\x`".into()
        }))
    );

    let mut parser = Parser::new("   \"foo");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 3,
            hi: 7,
            message: "expected closing `\"`, found EOF".into()
        }))
    );
}

#[test]
fn test_read_symbols() {
    let mut parser = Parser::new(
        r#"
foo
+foo
-foo
.foo
.*+!-_?$%&=<>:#123
+
-
namespaced/symbol
/
"#,
    );
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("+foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("-foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol(".foo".into()))));
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Symbol(".*+!-_?$%&=<>:#123".into())))
    );
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("+".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("-".into()))));
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Symbol("namespaced/symbol".into())))
    );
    assert_eq!(parser.read(), Some(Ok(Value::Symbol("/".into()))));
    assert_eq!(parser.read(), None);
}

#[test]
fn test_read_keywords() {
    let mut parser = Parser::new(
        r#"
:foo
:+foo
:-foo
:.foo
:.*+!-_?$%&=<>:#123
:+
:-
"#,
    );
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("+foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword("-foo".into()))));
    assert_eq!(parser.read(), Some(Ok(Value::Keyword(".foo".into()))));
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Keyword(".*+!-_?$%&=<>:#123".into())))
    );
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
    let mut parser = Parser::new(
        "() (1 2 3) (true, false, nil)
                                  (((\"foo\" \"bar\")))",
    );

    assert_eq!(parser.read(), Some(Ok(Value::List(vec![]))));

    assert_eq!(
        parser.read(),
        Some(Ok(Value::List(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ])))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::List(vec![
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Nil,
        ])))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::List(vec![Value::List(vec![Value::List(vec![
            Value::String("foo".into()),
            Value::String("bar".into()),
        ])])])))
    );

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("((  \\foo ))");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 4,
            hi: 8,
            message: "invalid char literal `\\foo`".into()
        }))
    );

    let mut parser = Parser::new("( (  1 2 3");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 2,
            hi: 10,
            message: "unclosed `(`".into()
        }))
    );
}

#[test]
fn test_read_vectors() {
    let mut parser = Parser::new(
        "[] [1 2 3] [true, false, nil]
                                  [[[\"foo\" \"bar\"]]]",
    );

    assert_eq!(parser.read(), Some(Ok(Value::Vector(vec![]))));

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Vector(vec![
            Value::Integer(1),
            Value::Integer(2),
            Value::Integer(3),
        ])))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Vector(vec![
            Value::Boolean(true),
            Value::Boolean(false),
            Value::Nil,
        ])))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Vector(vec![Value::Vector(vec![Value::Vector(
            vec![Value::String("foo".into()), Value::String("bar".into())],
        )])])))
    );

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("[[  \\foo ]]");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 4,
            hi: 8,
            message: "invalid char literal `\\foo`".into()
        }))
    );

    let mut parser = Parser::new("[ [  1 2 3");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 2,
            hi: 10,
            message: "unclosed `[`".into()
        }))
    );
}

#[test]
fn test_read_maps() {
    use std::collections::BTreeMap;

    let mut parser = Parser::new(
        "{} {1 2} {true, false}
                                  {{\"foo\" \"bar\"} \"baz\"}",
    );

    assert_eq!(parser.read(), Some(Ok(Value::Map(BTreeMap::new()))));

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Map({
            let mut map = BTreeMap::new();
            map.insert(Value::Integer(1), Value::Integer(2));
            map
        })))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Map({
            let mut map = BTreeMap::new();
            map.insert(Value::Boolean(true), Value::Boolean(false));
            map
        })))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Map({
            let mut map = BTreeMap::new();
            map.insert(
                Value::Map({
                    let mut map = BTreeMap::new();
                    map.insert(Value::String("foo".into()), Value::String("bar".into()));
                    map
                }),
                Value::String("baz".into()),
            );
            map
        })))
    );

    let mut parser = Parser::new("{\\foo true}");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 1,
            hi: 5,
            message: "invalid char literal `\\foo`".into()
        }))
    );

    let mut parser = Parser::new("{ { 1 2 3");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 2,
            hi: 9,
            message: "unclosed `{`".into()
        }))
    );

    let mut parser = Parser::new("{1 2 3}");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 0,
            hi: 7,
            message: "odd number of items in a Map".into()
        }))
    );

    let mut parser = Parser::new("{{1 2 3}}");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 1,
            hi: 8,
            message: "odd number of items in a Map".into()
        }))
    );
}

#[test]
fn test_read_sets() {
    let mut parser = Parser::new(
        "#{} #{1 2 2 3 3 3} #{true, false, nil}
                                  #{#{#{\"foo\" \"bar\"}}}",
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Set([].iter().cloned().collect())))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Set(
            [Value::Integer(1), Value::Integer(2), Value::Integer(3)]
                .iter()
                .cloned()
                .collect()
        )))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Set(
            [Value::Boolean(true), Value::Boolean(false), Value::Nil]
                .iter()
                .cloned()
                .collect()
        )))
    );

    assert_eq!(
        parser.read(),
        Some(Ok(Value::Set(
            [Value::Set(
                [Value::Set(
                    [Value::String("foo".into()), Value::String("bar".into())]
                        .iter()
                        .cloned()
                        .collect()
                )].iter()
                    .cloned()
                    .collect()
            )].iter()
                .cloned()
                .collect()
        )))
    );

    assert_eq!(parser.read(), None);

    let mut parser = Parser::new("#{#{  \\foo }}");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 6,
            hi: 10,
            message: "invalid char literal `\\foo`".into()
        }))
    );

    let mut parser = Parser::new("#{ #{ 1 2 3");
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 3,
            hi: 11,
            message: "unclosed `#{`".into()
        }))
    );
}

#[test]
fn test_tagged_values() {
    let mut parser = Parser::new(
        r#"
#color (255, 31, 191)
#foo/bar :baz
#nested #tags "works"
#noclose
"#,
    );
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Tagged(
            "color".into(),
            Box::new(Value::List(vec![
                Value::Integer(255),
                Value::Integer(31),
                Value::Integer(191),
            ]))
        )))
    );
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Tagged(
            "foo/bar".into(),
            Box::new(Value::Keyword("baz".into()))
        )))
    );
    assert_eq!(
        parser.read(),
        Some(Ok(Value::Tagged(
            "nested".into(),
            Box::new(Value::Tagged(
                "tags".into(),
                Box::new(Value::String("works".into()))
            ))
        )))
    );
    assert_eq!(
        parser.read(),
        Some(Err(Error {
            lo: 60,
            hi: 68,
            message: "malformed tagged value".into(),
        }))
    );

    assert_eq!(parser.read(), None);
}
