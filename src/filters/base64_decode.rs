use base64::Engine;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "base64_decode",
    description = "Decode a base64 encoded string.",
    parsed(Base64DecodeFilter)
)]
pub(crate) struct Base64Decode;

#[derive(Default, Display_filter)]
#[name = "output"]
struct Base64DecodeFilter;

impl std::fmt::Debug for Base64DecodeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutputFilter").finish()
    }
}

impl Filter for Base64DecodeFilter {
    fn evaluate(&self, input: &dyn ValueView, _runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_value();

        let encoded = String::from_utf8(
            base64::engine::general_purpose::STANDARD
                .decode(&input.to_kstr())
                .map_err(|e| liquid_core::Error::with_msg(e.to_string()))?,
        )
        .map_err(|e| liquid_core::Error::with_msg(e.to_string()))?;

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
    #[case(json!({"base64":"{{ 'VGhpcyBpcyBteSBtZXNzYWdl' | base64_decode }}"}), json!({}), json!({"base64": "This is my message"}))]
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
