# cloudevents 0.2.0

Implementation of the core in version:

* **v1.0** (see [v1.0 CloudEvents specification](https://github.com/cloudevents/spec/blob/v1.0/spec.md) and [v1.0 JSON Event Format](https://github.com/cloudevents/spec/blob/v1.0/json-format.md))
* **v0.2** (see [v0.2 CloudEvents specification](https://github.com/cloudevents/spec/blob/v0.2/spec.md) and [v0.2 JSON Event Format](https://github.com/cloudevents/spec/blob/v0.2/json-format.md))

This library is meant to provide the base for other CloudEvent transport bindings and formats. It only implements the core specification and the JSON format.

## Create CloudEvent according to latest spec

A cloud event can be create in two different ways:

### Using the macro

```rust
use cloudevents::cloudevent;
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::CloudEvent;
use failure::Error;

let event: Result<CloudEvent, Error> = cloudevent!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    time: "2019-12-04T18:33:09+00:00",
    subject: "me",
    dataschema: "https://lol.org/schema.json"
    datacontenttype: "application/json",
    data: Data::from_string("\"test\""),
);
```

### Using the builder

```rust
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::CloudEventLatest;
use failure::Error;

let event: Result<CloudEventLatest, Error> = CloudEventBuilder::latest()
    .event_id("id")
    .source("http://www.google.com")
    .event_type("test type")
    .datacontenttype("application/json")
    .data(Data::from_string("\"test\""))
    .build();
```

Check out the module documentation to learn about version specific macros and builders.

## JSON encoding

The CloudEvents can be serialized/deserialized with `serde_json`

### Serialization

```rust
use serde_json;
use cloudevents::cloudevent;
use cloudevents::Data;

let event = cloudevent!(
  event_type: "test type",
  source: "http://www.google.com",
  event_id: "id",
  datacontenttype: "application/json",
  data: Data::from_string("\"test\""),
);

let json = serde_json::to_string(&event.unwrap()).unwrap();
assert_eq!(json, "{\"type\":\"test type\",\"specversion\":\"1.0\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"datacontenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}");
```

### Deserialization

```rust
use serde_json;
use cloudevents::cloudevent_v1_0;
use cloudevents::{Data, CloudEvent};

let data = "{\"type\":\"test type\",\"specversion\":\"1.0\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"datacontenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}";
let expected_event = cloudevent_v1_0!(
  event_type: "test type",
  source: "http://www.google.com",
  event_id: "id",
  datacontenttype: "application/json",
  data: Data::from_string("\"test\""),
).unwrap();

let event: CloudEvent = serde_json::from_str(data).unwrap();

match event {
  CloudEvent::V1_0(event) => assert_eq!(event, expected_event),
  _ => assert!(false)
}
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

## Update Readme

The original readme text is an rust doc comment in the [lib.rs](./cloudevents/src/lib.rs) file

1. cargo install cargo-readme
2. cargo readme  -r cloudevents > README.md
