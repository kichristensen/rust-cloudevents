/*!
Implementation of the core in version:

* **v1.0** (see [v1.0 CloudEvents specification](https://github.com/cloudevents/spec/blob/v1.0/spec.md) and [v1.0 JSON Event Format](https://github.com/cloudevents/spec/blob/v1.0/json-format.md))
* **v0.2** (see [v0.2 CloudEvents specification](https://github.com/cloudevents/spec/blob/v0.2/spec.md) and [v0.2 JSON Event Format](https://github.com/cloudevents/spec/blob/v0.2/json-format.md))

This library is meant to provide the base for other CloudEvent transport bindings and formats. It only implements the core specification and the JSON format.

# Usage v1.0

A cloud event can be create in two different ways:

## Using the builder

```
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v1_0::CloudEventV1_0;
use failure::Error;

let event : Result<CloudEventV1_0, Error> = CloudEventBuilder::default() // or CloudEventBuilder::v1_0()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .datacontenttype("application/json")
  .data(Data::from_string("\"test\""))
  .build();
```

## Using the macro

```
use cloudevents::cloudevent_v1_0;
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v1_0::CloudEventV1_0;
use failure::Error;

let event : Result<CloudEventV1_0, Error> = cloudevent_v1_0!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    datacontenttype: "application/json",
    data: Data::from_string("\"test\""),
);
```


# Usage with spec version 0.2

A cloud event can be create in two different ways:

## Using the builder

```
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v0_2::CloudEventV0_2;
use failure::Error;

let event : Result<CloudEventV0_2, Error> = CloudEventBuilder::v0_2()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .contenttype("application/json")
  .data(Data::from_string("\"test\""))
  .build();
```

## Using the macro

```
use cloudevents::cloudevent_v0_2;
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v0_2::CloudEventV0_2;
use failure::Error;

let event : Result<CloudEventV0_2, Error> = cloudevent_v0_2!(
    event_type: "test type",
    source: "http://www.google.com",
    event_id: "id",
    contenttype: "application/json",
    data: Data::from_string("\"test\""),
);
```

# To serialize the event as JSON, just use `serde_json`

```
use serde_json;
use cloudevents::cloudevent_v1_0;
use cloudevents::Data;

let event = cloudevent_v1_0!(
  event_type: "test type",
  source: "http://www.google.com",
  event_id: "id",
  datacontenttype: "application/json",
  data: Data::from_string("\"test\""),
);

let json = serde_json::to_string(&event.unwrap()).unwrap();
assert_eq!(json, "{\"type\":\"test type\",\"specversion\":\"1.0\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"datacontenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}");
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

mod builder;
mod common;
mod event;
pub mod helper;
pub mod v0_2;
pub mod v1_0;

pub use crate::builder::CloudEventBuilder;
pub use crate::common::{Data, ExtensionValue};
pub use crate::event::CloudEvent;
