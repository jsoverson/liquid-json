use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};
use serde::de::DeserializeOwned;

use super::invalid_input;

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "json",
    description = "Parses a JSON string into a JSON object.",
    parsed(JsonFilter)
)]
pub(crate) struct Json;

#[derive(Default, Display_filter)]
#[name = "json"]
struct JsonFilter;

impl std::fmt::Debug for JsonFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonFilter").finish()
    }
}

impl Filter for JsonFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_value();

        jsonify(input)
    }
}

fn jsonify(input: Value) -> Result<Value> {
    Ok(match input {
        Value::Scalar(v) => {
            let view = v.as_view();
            match view.type_name() {
                "string" => parse_json(v.to_kstr())?,
                _ => Value::Scalar(v),
            }
        }
        Value::Array(v) => Value::Array(v.into_iter().map(jsonify).collect::<Result<_, _>>()?),
        Value::Object(v) => Value::Object(
            v.into_iter()
                .map(|(k, v)| (k, jsonify(v).unwrap()))
                .collect(),
        ),
        Value::State(_) => unreachable!(),
        Value::Nil => input,
    })
}

fn parse_json<T: DeserializeOwned>(src: impl AsRef<str>) -> Result<T> {
    serde_json::from_str(src.as_ref())
        .map_err(|e| invalid_input(format!("invalid JSON string: {}", e)))
}

#[cfg(test)]
mod tests {
    use crate::LiquidJson;
    use rstest::rstest;
    use serde_json::{json, Value};

    use anyhow::Result;

    #[rstest]
    #[case(json!({"this":"{{ myval | each: '{\"height\":{{el}}}' | json | output }}"}), json!({"myval": [5.1,4.2]}), json!({"this":[{"height": 5.1},{"height": 4.2}]}))]
    #[case(json!({"recipients" : "{{ to | each: '{ \"email\": \"{{ el }}\" }' | json | output }}"}), json!({"to": ["john@example.com", "jane@example.com"]}), json!({"recipients": [
      {
        "email": "john@example.com"
      },
      {
        "email": "jane@example.com"
      }
    ]}))]
    fn filters(
        #[case] template: Value,
        #[case] data: Value,
        #[case] expected: Value,
    ) -> Result<()> {
        let tmpl = LiquidJson::new(template);
        let actual = tmpl.render(&data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
