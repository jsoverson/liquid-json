use crate::PARSER;
use liquid_core::parser::FilterArguments;
use liquid_core::Expression;
use liquid_core::Object;
use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{
    Display_filter, Filter, FilterParameters, FilterReflection, FromFilterParameters, ParseFilter,
};
use liquid_core::{Value, ValueView};

use super::invalid_input;

#[derive(Debug, FilterParameters)]
struct EachArgs {
    #[parameter(
        description = "The template to apply to each element.",
        arg_type = "str"
    )]
    template: Expression,
}

#[derive(Clone, FilterReflection)]
#[filter(
    name = "each",
    description = "Iterates over an array, applying a template to each element.",
    parameters(EachArgs),
    parsed(EachFilter)
)]
pub(crate) struct Each {}

impl ParseFilter for Each {
    fn parse(&self, arguments: FilterArguments) -> Result<Box<dyn Filter>> {
        let args = EachArgs::from_args(arguments)?;

        Ok(Box::new(EachFilter { args }))
    }

    fn reflection(&self) -> &dyn FilterReflection {
        self
    }
}

impl Each {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

#[derive(FromFilterParameters, Display_filter)]
#[name = "each"]
struct EachFilter {
    #[parameters]
    args: EachArgs,
}

impl std::fmt::Debug for EachFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EachFilter")
            .field("args", &self.args)
            .finish()
    }
}

impl Filter for EachFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let args = self.args.evaluate(runtime)?;

        let template = PARSER.parse(&args.template)?;

        let input = input
            .as_array()
            .ok_or_else(|| invalid_input("Array expected"))?;

        let output: Vec<_> = input
            .values()
            .map(|x| {
                let mut global = Object::new();
                global.insert("el".into(), x.to_value());
                Ok(Value::Scalar(template.render(&global)?.into()))
            })
            .collect::<Result<_, _>>()?;
        Ok(Value::array(output))
    }
}
