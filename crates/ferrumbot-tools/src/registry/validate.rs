use jsonschema::validator_for;
use serde_json::Value;

pub(super) fn validate_params(schema: Value, args: &Value) -> Vec<String> {
    let validator = match validator_for(&schema) {
        Ok(v) => v,
        Err(err) => return vec![format!("invalid schema: {err}")],
    };

    validator
        .iter_errors(args)
        .map(|e| {
            let path = e.instance_path.to_string();
            if path.is_empty() {
                format!("parameter {}", e)
            } else {
                format!("{path} {e}")
            }
        })
        .collect()
}
