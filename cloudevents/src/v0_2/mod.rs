/*!

# Macro Usage

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

# Builder Usage

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
 */
mod builder;
mod event;
mod helper;

pub use self::builder::CloudEventV0_2Builder;
pub use self::event::CloudEventV0_2;
