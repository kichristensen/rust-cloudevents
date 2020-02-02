/*!

# Macro Usage

```
use cloudevents::cloudevent_v1_0;
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v1_0::CloudEventV1_0;
use failure::Error;

let event : Result<CloudEventV1_0, Error> = cloudevent_v1_0!(
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

# Builder Usage

```
use cloudevents::{Data, CloudEventBuilder};
use cloudevents::v1_0::CloudEventV1_0;
use failure::Error;

let event : Result<CloudEventV1_0, Error> = CloudEventBuilder::v1_0()
  .event_id("id")
  .source("http://www.google.com")
  .event_type("test type")
  .build();
```
 */
mod builder;
mod event;
mod helper;

pub use self::builder::CloudEventV1_0Builder;
pub use self::event::CloudEventV1_0;
