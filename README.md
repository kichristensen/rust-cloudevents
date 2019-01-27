# CloudEvents

[![Build Status](https://travis-ci.org/kichristensen/rust-cloudevents.svg?branch=master)](https://travis-ci.org/kichristensen/rust-cloudevents)

Implementation of the core [v0.2 CloudEvents specification](https://github.com/cloudevents/spec/blob/v0.2/spec.md) and [v0.2 JSON Event Format](https://github.com/cloudevents/spec/blob/v0.2/json-format.md).

This library is meant to provide the base for other CloudEvent transport bindings and formats. It only implements the core specification and the JSON format.

## Usage

A cloud event can be create in two different ways:

```rust
use cloudevents::{
  cloudevent_v02,
  v02::{CloudEventBuilder}
};
use std::error::Error;

// Using the builder
let event : Result<CloudEvent, Error> = CloudEventBuilder::default()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .contenttype("application/json")
  .build();

// or using the macro
let event : Result<CloudEvent, Error> = cloudevent!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    contenttype: "application/json",
    data: Data::from_string("test"),
)
```

To serialize the event as JSON, just use `serde_json`:

```rust
let json = serde_json::to_string(&event)?;
```

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.