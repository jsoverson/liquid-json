use crate::{render_value, to_liquid_obj, Error};

/// A JSON structured Liquid template.
#[must_use]
#[derive(Clone)]
pub struct LiquidJson {
    pub(crate) raw_template: serde_json::Value,
}

impl std::fmt::Debug for LiquidJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LiquidTemplate")
            .field("template", &self.raw_template)
            .finish()
    }
}

impl PartialEq for LiquidJson {
    fn eq(&self, other: &Self) -> bool {
        self.raw_template == other.raw_template
    }
}

impl LiquidJson {
    /// Create a new Liquid template from a JSON value.
    pub fn new(raw_template: serde_json::Value) -> Self {
        LiquidJson { raw_template }
    }

    /// Render the Liquid JSON template with the given data.
    pub fn render(&self, data: &serde_json::Value) -> Result<serde_json::Value, Error> {
        let data = to_liquid_obj(data)?;
        render_value(&self.raw_template, &data)
    }

    /// Get the inner [serde_json::Value] value.
    pub fn as_json(&self) -> &serde_json::Value {
        &self.raw_template
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::{json, Value};

    use super::*;
    use anyhow::Result;

    #[rstest]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": 5}), json!({"this":5}))]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": f64::MAX}), json!({"this":f64::MAX  }))]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": i64::MAX}), json!({"this":i64::MAX}))]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": u32::MAX}), json!({"this":u32::MAX}))]
    // #[case(json!({"this":"{{ myval }}"}), json!({"myval": u64::MAX}), json!({"this":u64::MAX}))] // Fails for now...
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": "5"}), json!({"this":"5"}))]
    #[case(json!({"{{ myval }}":"bar"}), json!({"myval": "this"}), json!({"this":"bar"}))]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": 5.1}), json!({"this":5.1}))]
    #[case(json!({"this":"{{ myval }}"}), json!({"myval": [5.1,4.2]}), json!({"this":[5.1,4.2]}))]
    #[case(json!({"this":"{{ myval | each: \"my num: {{el}}\" | output}}"}), json!({"myval": [5.1,4.2]}), json!({"this":["my num: 5.1","my num: 4.2"]}))]
    #[case(json!({"this":"{{ myval | default: 'hey'}}"}), json!({}), json!({"this":"hey"}))]
    fn basic(#[case] template: Value, #[case] data: Value, #[case] expected: Value) -> Result<()> {
        let tmpl = LiquidJson::new(template);
        let actual = tmpl.render(&data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
