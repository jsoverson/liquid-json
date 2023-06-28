use base64::Engine;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_encode",
    description = "Encode a string as base64",
    parsed(Base64EncodeFilter)
)]
pub(crate) struct Base64Encode;

#[derive(Default, Display_filter)]
#[name = "output"]
struct Base64EncodeFilter;

impl std::fmt::Debug for Base64EncodeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutputFilter").finish()
    }
}

impl Filter for Base64EncodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_value();

        let encoded = base64::engine::general_purpose::STANDARD
            .encode(&input.to_kstr())
            .to_string();

        Ok(Value::Scalar(encoded.into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::LiquidJson;
    use rstest::rstest;
    use serde_json::{json, Value};

    use anyhow::Result;

    #[rstest]
    #[case(json!({"base64":"{{ 'This is my message' | base64_encode }}"}), json!({}), json!({"base64": "VGhpcyBpcyBteSBtZXNzYWdl"}))]
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
