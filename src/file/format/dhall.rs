use std::collections::HashMap;
use std::error::Error;

use crate::value::{Value, ValueKind};

pub fn parse(
    uri: Option<&String>,
    text: &str,
) -> Result<HashMap<String, Value>, Box<dyn Error + Send + Sync>> {
    // Parse a Dhall object value from the text
    // TODO: Have a proper error fire if the root of a file is ever not a Table
    let value = from_dhall_value(uri, serde_dhall::from_str(text).parse()?);
    match value.kind {
        ValueKind::Table(map) => Ok(map),

        _ => Ok(HashMap::new()),
    }
}

fn from_dhall_value(uri: Option<&String>, value: serde_dhall::SimpleValue) -> Value {
    match value {
        serde_dhall::SimpleValue::Num(num) => match num {
            serde_dhall::NumKind::Bool(b) => Value::new(uri, ValueKind::Boolean(b)),
            serde_dhall::NumKind::Natural(n) => Value::new(uri, ValueKind::Integer(n as i64)),
            serde_dhall::NumKind::Integer(i) => Value::new(uri, ValueKind::Integer(i)),
            serde_dhall::NumKind::Double(d) => Value::new(uri, ValueKind::Float(f64::from(d))),
        },
        serde_dhall::SimpleValue::Text(string) => Value::new(uri, ValueKind::String(string)),
        serde_dhall::SimpleValue::List(list) => Value::new(
            uri,
            ValueKind::Array(list.into_iter().map(|v| from_dhall_value(uri, v)).collect()),
        ),
        serde_dhall::SimpleValue::Record(rec) => Value::new(
            uri,
            ValueKind::Table(
                rec.into_iter()
                    .map(|(k, v)| (k, from_dhall_value(uri, v)))
                    .collect(),
            ),
        ),
        serde_dhall::SimpleValue::Optional(Some(value))
        | serde_dhall::SimpleValue::Union(_, Some(value)) => from_dhall_value(uri, *value),
        serde_dhall::SimpleValue::Optional(None) | serde_dhall::SimpleValue::Union(_, None) => {
            Value::new(uri, ValueKind::Nil)
        }
    }
}
