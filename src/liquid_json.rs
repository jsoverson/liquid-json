use crate::{render_value, to_liquid_obj, Error};

/// A JSON structured Liquid template.
#[must_use]
pub struct LiquidJson {
    pub(crate) raw_template: serde_json::Value,
    parser: liquid::Parser,
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
        let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        LiquidJson {
            raw_template,
            parser,
        }
    }

    /// Render the Liquid JSON template with the given data.
    pub fn render(&self, data: serde_json::Value) -> Result<serde_json::Value, Error> {
        let data = to_liquid_obj(data)?;
        render_value(&self.parser, &self.raw_template, &data)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::{json, Value};

    use super::*;
    use anyhow::Result;

    #[rstest]
    #[case(json!({"this":"{{myval}}"}), json!({"myval": 5}), json!({"this":5}))]
    #[case(json!({"this":"{{myval}}"}), json!({"myval": "5"}), json!({"this":"5"}))]
    #[case(json!({"this":"{{myval}}"}), json!({"myval": 5.1}), json!({"this":5.1}))]
    #[case(json!({"this":"{{myval}}"}), json!({"myval": [5.1,4.2]}), json!({"this":[5.1,4.2]}))]
    fn basic(#[case] template: Value, #[case] data: Value, #[case] expected: Value) -> Result<()> {
        let tmpl = LiquidJson::new(template);
        let actual = tmpl.render(data)?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
