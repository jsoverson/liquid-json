# Liquid JSON template library

This library is a small wrapper around the [Liquid](https://shopify.github.io/liquid/) templating engine that recursively processes structured JSON values for Liquid templates.

Liquid JSON templates help templatize JSON files used in configuration or RPC transmission.

## Usage

```rust
use serde_json::json;
let template_json = json!({"this":"{{myval}}"});
let template_data = json!({"myval": 5});
let tmpl = LiquidJson::new(template_json);
let actual = tmpl.render(template_data)?;

let expected = json!({"this": 5}); // {{myval}} is replaced with 5
assert_eq!(actual, expected);
```

## Features

Turn on the `serde` feature to expose `LiquidJsonValue`. `LiquidJsonValue` is a wrapper around `LiquidJson` (and `serde_json::Value`) that lets you embed `LiquidJson` templates in your structs, e.g.

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct YourStruct {
    inner_liquid: LiquidJsonValue,
}

let json_data = json!({"inner_liquid":"{{myval}}"});

let template_data = json!({"myval": 5});

let yours: YourStruct = serde_json::from_value(from_json.clone())?;
let actual = yours.inner_liquid.render(data)?;

```
