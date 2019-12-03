/*!
Implementation of the core in version

* v1.0: [v1.0 CloudEvents specification](https://github.com/cloudevents/spec/blob/v1.0/spec.md) and [v1.0 JSON Event Format](https://github.com/cloudevents/spec/blob/v1.0/json-format.md).
* v0.2: [v0.2 CloudEvents specification](https://github.com/cloudevents/spec/blob/v0.2/spec.md) and [v0.2 JSON Event Format](https://github.com/cloudevents/spec/blob/v0.2/json-format.md).

This library is meant to provide the base for other CloudEvent transport bindings and formats. It only implements the core specification and the JSON format.

# Usage v1.0

A cloud event can be create in two different ways:

```
#[macro_use]
use cloudevents::{cloudevent_v1_0, Data};
use cloudevents::v1_0::{CloudEventBuilder, CloudEvent};
use failure::Error;

// Using the builder
let event : Result<CloudEvent, Error> = CloudEventBuilder::default()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .datacontenttype("application/json")
  .build();

// or using the macro
let event : Result<CloudEvent, Error> = cloudevent_v1_0!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    datacontenttype: "application/json",
    data: Data::from_string("\"test\""),
);

// To serialize the event as JSON, just use `serde_json`:

use serde_json;
let json = serde_json::to_string(&event.unwrap()).unwrap();
assert_eq!(json, "{\"type\":\"test type\",\"specversion\":\"1.0\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"datacontenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}");
```

# Usage v0.2

A cloud event can be create in two different ways:

```
#[macro_use]
use cloudevents::{cloudevent_v02, Data};
use cloudevents::v02::{CloudEventBuilder, CloudEvent};
use failure::Error;

// Using the builder
let event : Result<CloudEvent, Error> = CloudEventBuilder::default()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .contenttype("application/json")
  .build();

// or using the macro
let event : Result<CloudEvent, Error> = cloudevent_v02!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    contenttype: "application/json",
    data: Data::from_string("\"test\""),
);

// To serialize the event as JSON, just use `serde_json`:

use serde_json;
let json = serde_json::to_string(&event.unwrap()).unwrap();
assert_eq!(json, "{\"type\":\"test type\",\"specversion\":\"0.2\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"contenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}");
```

# License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
*/

mod common;
pub mod v02;
pub mod v1_0;

pub use crate::common::{Data, ExtensionValue};
