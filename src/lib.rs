//! Liquid JSON templates
#![doc = include_str!("../README.md")]
#![allow(unknown_lints)]
#![deny(
    clippy::expect_used,
    clippy::explicit_deref_methods,
    clippy::option_if_let_else,
    clippy::await_holding_lock,
    clippy::cloned_instead_of_copied,
    clippy::explicit_into_iter_loop,
    clippy::flat_map_option,
    clippy::fn_params_excessive_bools,
    clippy::implicit_clone,
    clippy::inefficient_to_string,
    clippy::large_types_passed_by_value,
    clippy::manual_ok_or,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::must_use_candidate,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::redundant_else,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::unnested_or_patterns,
    clippy::future_not_send,
    clippy::useless_let_if_seq,
    clippy::str_to_string,
    clippy::inherent_to_string,
    clippy::let_and_return,
    clippy::string_to_string,
    clippy::try_err,
    clippy::unused_async,
    clippy::missing_enforced_import_renames,
    clippy::nonstandard_macro_braces,
    clippy::rc_mutex,
    clippy::unwrap_or_else_default,
    clippy::manual_split_once,
    clippy::derivable_impls,
    clippy::needless_option_as_deref,
    clippy::iter_not_returning_iterator,
    clippy::same_name_method,
    clippy::manual_assert,
    clippy::non_send_fields_in_send_ty,
    clippy::equatable_if_let,
    bad_style,
    clashing_extern_declarations,
    dead_code,
    deprecated,
    explicit_outlives_requirements,
    improper_ctypes,
    invalid_value,
    missing_copy_implementations,
    missing_debug_implementations,
    mutable_transmutes,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    unconditional_recursion,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    while_true,
    missing_docs
)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::box_default)]

mod error;
mod filters;
mod liquid_json;
#[cfg(feature = "serde")]
mod liquid_json_value;
mod options;

use std::sync::Arc;

pub use error::Error;
use liquid::{Parser, ValueView};
use liquid_core::{
    model::ScalarCow,
    runtime::{RuntimeBuilder, Variable},
    Language, Runtime,
};
#[cfg(feature = "serde")]
pub use liquid_json_value::LiquidJsonValue;
use once_cell::sync::Lazy;
use serde_json::Number;

pub use crate::liquid_json::LiquidJson;

use self::options::OptionsBuilder;

static PARSER: Lazy<Arc<Parser>> = Lazy::new(|| {
    let builder = liquid::ParserBuilder::with_stdlib()
        .filter(filters::Each::new())
        .filter(filters::Output)
        .filter(filters::Base64Decode)
        .filter(filters::Base64Encode);
    #[cfg(feature = "serde")]
    let builder = builder.filter(filters::Json);
    Arc::new(builder.build().unwrap())
});

static OPTIONS: Lazy<Arc<Language>> = Lazy::new(|| {
    let builder = OptionsBuilder::new()
        .stdlib()
        .filter(filters::Each::new())
        .filter(filters::Output)
        .filter(filters::Base64Decode)
        .filter(filters::Base64Encode);
    #[cfg(feature = "serde")]
    let builder = builder.filter(filters::Json);
    builder.build()
});

/// Utility function to render a basic string with a [serde_json::Value] instead of dealing with [liquid::Object].
pub fn render_string(template: &str, data: &serde_json::Value) -> Result<String, Error> {
    let data = to_liquid_obj(data)?;
    inner_render_string(template, &data)
}

fn inner_render_string(template: &str, data: &liquid::Object) -> Result<String, Error> {
    let template = PARSER.parse(template)?;
    Ok(template.render(data)?)
}

fn to_liquid_obj(value: &serde_json::Value) -> Result<liquid::Object, Error> {
    // let mut obj = liquid::Object::new();
    match value {
        serde_json::Value::Object(v) => v
            .into_iter()
            .map(|(k, v)| {
                Ok((
                    liquid::model::KString::from_string(k.clone()),
                    to_liquid_value(v)?,
                ))
            })
            .collect::<Result<liquid::Object, Error>>(),
        _ => Err(Error::InvalidContext(value.clone())),
    }
}

fn to_liquid_value(value: &serde_json::Value) -> Result<liquid::model::Value, Error> {
    Ok(match value {
        serde_json::Value::Null => liquid::model::Value::Nil,
        serde_json::Value::Bool(v) => liquid::model::Value::Scalar(liquid::model::Scalar::from(*v)),
        serde_json::Value::Number(v) => {
            if v.is_f64() {
                liquid::model::Value::Scalar(liquid::model::Scalar::from(v.as_f64().unwrap()))
            } else if v.is_i64() {
                liquid::model::Value::Scalar(liquid::model::Scalar::from(v.as_i64().unwrap()))
            } else {
                let num = v.as_u64().unwrap();
                if num < u32::MAX as u64 {
                    liquid::model::Value::Scalar(liquid::model::Scalar::new(ScalarCow::new(
                        v.as_u64().unwrap() as u32,
                    )))
                } else {
                    return Err(Error::U64);
                }
            }
        }
        serde_json::Value::String(v) => {
            liquid::model::Value::Scalar(liquid::model::Scalar::from(v.clone()))
        }
        serde_json::Value::Array(v) => {
            liquid::model::Value::Array(v.iter().map(to_liquid_value).collect::<Result<_, _>>()?)
        }
        serde_json::Value::Object(v) => liquid::model::Value::Object(
            v.into_iter()
                .map(|(k, v)| {
                    Ok((
                        liquid::model::KString::from_string(k.clone()),
                        to_liquid_value(v)?,
                    ))
                })
                .collect::<Result<liquid::model::Object, Error>>()?,
        ),
    })
}

fn to_json_value(value: liquid::model::Value) -> serde_json::Value {
    match value {
        liquid::model::Value::Scalar(v) => {
            // have to match on type name because liquid::model::Scalar doesn't expose its enum.
            let name = v.type_name();
            match name {
                "string" => serde_json::Value::String(v.to_kstr().to_string()),
                "whole number" => serde_json::Value::Number(Number::from(v.to_integer().unwrap())),
                "fractional number" => {
                    serde_json::Value::Number(Number::from_f64(v.to_float().unwrap()).unwrap())
                }
                "boolean" => serde_json::Value::Bool(v.to_bool().unwrap()),
                _ => panic!("Unknown scalar type: {}", name),
            }
        }
        liquid::model::Value::Array(v) => {
            serde_json::Value::Array(v.into_iter().map(to_json_value).collect())
        }
        liquid::model::Value::Object(v) => serde_json::Value::Object(
            v.into_iter()
                .map(|(k, v)| (k.to_string(), to_json_value(v)))
                .collect(),
        ),
        liquid::model::Value::State(_v) => panic!("State not supported"),
        liquid::model::Value::Nil => serde_json::Value::Null,
    }
}

static SINGLE_VALUE: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^\{\{\s*(\w*)\s*\}\}$").unwrap());

fn render_value(
    value: &serde_json::Value,
    data: &liquid::Object,
) -> Result<serde_json::Value, Error> {
    match value {
        serde_json::Value::String(s) => {
            // Special case: if the entire string is a single value, return that JSON value directly.
            if let Some(cap) = SINGLE_VALUE.captures(s) {
                let key = cap.get(1).unwrap().as_str();
                if let Some(val) = data.get(key) {
                    return Ok(to_json_value(val.clone()));
                }
            }
            let mut output = Vec::new();
            let runtime = RuntimeBuilder::new().set_globals(data).build();

            let elements = liquid_core::parser::parse(s, &OPTIONS)?;
            for element in elements {
                element.render_to(&mut output, &runtime)?;
            }
            let sentinel = Variable::with_literal("__output__");
            if let Some(output) = sentinel.try_evaluate(&runtime) {
                if let Some(value) = runtime.try_get(&output) {
                    return Ok(to_json_value(value.to_value()));
                }
            }
            let output = String::from_utf8(output).unwrap();
            Ok(serde_json::Value::String(output))
        }
        serde_json::Value::Array(a) => Ok(serde_json::Value::Array(
            a.iter()
                .map(|v| render_value(v, data))
                .collect::<Result<Vec<serde_json::Value>, _>>()?,
        )),
        serde_json::Value::Object(o) => {
            let map = o
                .into_iter()
                .map(|(k, v)| Ok((inner_render_string(k, data)?, render_value(v, data)?)))
                .collect::<Result<serde_json::Map<String, serde_json::Value>, Error>>()?;
            Ok(serde_json::Value::Object(map))
        }
        _ => Ok(value.clone()),
    }
}
