use serde::{Deserialize, Serialize};

use crate::{liquid_json::LiquidJson, Error};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
/// A Liquid JSON value that implements Serialize/Deserialize.
#[must_use]
pub struct LiquidJsonValue(
    #[serde(serialize_with = "ser_with", deserialize_with = "deser_with")] LiquidJson,
);

impl LiquidJsonValue {
    /// Create a new Liquid JSON value from a JSON value.
    pub fn new(raw_template: serde_json::Value) -> Self {
        LiquidJsonValue(LiquidJson::new(raw_template))
    }
    /// Render the JSON template with the given data.
    pub fn render(&self, data: &serde_json::Value) -> Result<serde_json::Value, Error> {
        self.0.render(data)
    }

    /// Get the inner [LiquidJson] value.
    pub fn inner(&self) -> &LiquidJson {
        &self.0
    }

    /// Get the unrendered template as a [serde_json::Value].
    pub fn as_json(&self) -> &serde_json::Value {
        self.0.as_json()
    }
}

impl From<serde_json::Value> for LiquidJsonValue {
    fn from(value: serde_json::Value) -> Self {
        LiquidJsonValue::new(value)
    }
}

fn ser_with<S>(tpl: &LiquidJson, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    tpl.raw_template.serialize(s)
}

fn deser_with<'de, D>(deserializer: D) -> Result<LiquidJson, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let raw_template = serde_json::Value::deserialize(deserializer)?;
    Ok(LiquidJson::new(raw_template))
}
#[cfg(test)]
mod tests {
    use ::serde::{Deserialize, Serialize};
    use rstest::rstest;
    use serde_json::{json, Value};

    use super::*;
    use anyhow::Result;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestSerde {
        inner_liquid: LiquidJsonValue,
    }

    #[rstest]
    #[case(json!({"inner_liquid":"{{myval}}"}), json!({"myval": 5}), json!(5))]
    #[case(json!({"inner_liquid":{"key":"{{myval}}"}}), json!({"myval": 5}), json!({"key":5}))]
    #[case(json!({"inner_liquid":{"key":"{{myval}}"}}), json!({"myval": {"deeper":10}}), json!({"key":{"deeper":10}}))]
    fn serde(#[case] from_json: Value, #[case] data: Value, #[case] expected: Value) -> Result<()> {
        let deser: TestSerde = serde_json::from_value(from_json.clone())?;
        let actual = deser.inner_liquid.render(&data)?;
        assert_eq!(actual, expected);
        let to_json = serde_json::to_value(deser)?;
        assert_eq!(to_json, from_json);
        Ok(())
    }
}
