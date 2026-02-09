use serde_json::Value;

pub(crate) fn normalize_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let converted = map
                .into_iter()
                .map(|(k, v)| (camel_to_snake(&k), normalize_keys(v)))
                .collect();
            Value::Object(converted)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(normalize_keys).collect()),
        other => other,
    }
}

pub(crate) fn to_camel_case_keys(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let converted = map
                .into_iter()
                .map(|(k, v)| (snake_to_camel(&k), to_camel_case_keys(v)))
                .collect();
            Value::Object(converted)
        }
        Value::Array(items) => Value::Array(items.into_iter().map(to_camel_case_keys).collect()),
        other => other,
    }
}

fn camel_to_snake(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 5);
    for (idx, ch) in input.chars().enumerate() {
        if ch.is_ascii_uppercase() && idx > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn snake_to_camel(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut upper = false;
    for ch in input.chars() {
        if ch == '_' {
            upper = true;
            continue;
        }
        if upper {
            out.push(ch.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(ch);
        }
    }
    out
}
