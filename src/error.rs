#[derive(Debug, thiserror::Error)]
/// Errors that can occur when rendering a Liquid JSON template.
pub enum Error {
    /// Thrown when the data provided to render functions isn't a Key/Value map.
    #[error("Invalid context passed to template. Expected a map, got {0:?}")]
    InvalidContext(serde_json::Value),
    /// Passed through from the Liquid library.
    #[error(transparent)]
    LiquidError(#[from] liquid::Error),
    /// Tried to use a u64 value in a Liquid template, which isn't supported by the Liquid library.
    #[error("Liquid templates do not support u64 values as of right now")]
    U64,
}
