extern crate itertools;

use self::itertools::Itertools;
use Value;

pub fn print(v: Value) -> String {
    match v {
        Value::Integer(n) => n.to_string(),
        Value::Nil => "nil".to_string(),
        Value::Boolean(n) => (if n { "true" } else { "false" }).to_string(),
        Value::String(n) => format!("\"{}\"", n.replace("\"", "\\\"")),
        Value::Char(n) => format!("\\{}", n),
        Value::Symbol(n) => n.to_string(),
        Value::Keyword(n) => format!(":{}", n),
        Value::Float(n) => n.to_string(),
        Value::List(n) => format!("({})", n.into_iter().map(|v| print(v)).join(", ")),
        Value::Vector(n) => format!("[{}]", n.into_iter().map(|v| print(v)).join(", ")),
        Value::Map(n) => {
            let mut content = n.iter().map(|e| {
                let (k, v) = e;
                return format!("{} {}", print(k.clone()), print(v.clone()));
            });
            return format!("{}{}{}", "{", content.join(", "), "}");
        }
        Value::Set(n) => format!("#{}{}{}", "{", n.iter().map(|e| print(e.clone())).join(", "), "}"),
        Value::Tagged(t, v) => format!("#{} {}", t, print(*v)),
    }
}
