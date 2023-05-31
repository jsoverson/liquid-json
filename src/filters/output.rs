use liquid_core::Result;
use liquid_core::Runtime;
use liquid_core::{Display_filter, Filter, FilterReflection, ParseFilter};
use liquid_core::{Value, ValueView};

#[derive(Clone, ParseFilter, FilterReflection)]
#[filter(
    name = "output",
    description = "Marks a value as the output of the template.",
    parsed(OutputFilter)
)]
pub(crate) struct Output;

#[derive(Default, Display_filter)]
#[name = "output"]
struct OutputFilter;

impl std::fmt::Debug for OutputFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OutputFilter").finish()
    }
}

impl Filter for OutputFilter {
    fn evaluate(&self, input: &dyn ValueView, runtime: &dyn Runtime) -> Result<Value> {
        let input = input.to_value();

        runtime.set_global("__output__".into(), input);

        Ok(Value::Nil)
    }
}
