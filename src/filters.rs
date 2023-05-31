mod each;
#[cfg(feature = "serde")]
mod json;
mod output;

pub(crate) use each::Each;
#[cfg(feature = "serde")]
pub(crate) use json::Json;
pub(crate) use output::Output;

pub(crate) fn invalid_input<S>(cause: S) -> liquid_core::Error
where
    S: Into<liquid_core::model::KString>,
{
    liquid_core::Error::with_msg("Invalid input").context("cause", cause)
}

#[cfg(test)]
mod tests {
    use crate::LiquidJson;
    use rstest::rstest;
    use serde_json::{json, Value};

    use anyhow::Result;

    #[rstest]
    #[case(json!({"this":"{{ myval | output }}"}), json!({"myval": [5.1,4.2]}), json!({"this":[5.1,4.2]}))]
    #[case(json!({"this":"{{ myval | each: \"I am {{el}} feet tall\" | output }}"}), json!({"myval": [5.1,4.2]}), json!({"this":["I am 5.1 feet tall","I am 4.2 feet tall"]}))]
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
