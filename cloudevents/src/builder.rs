use crate::v0_2::CloudEventV0_2Builder;
use crate::v1_0::CloudEventV1_0Builder;

type DefaultCloudEventBuilder = CloudEventV1_0Builder;

/// Create a new `CloudEvent` in the desired spec version.
///
/// # Example
/// 
/// ```
/// use cloudevents::CloudEventBuilder;
///
/// let event= CloudEventBuilder::v1_0()
///     .event_id("id")
///     .source("http://www.google.com")
///     .event_type("test type")
///     .datacontenttype("application/json")
///     .build();
/// ```
pub struct CloudEventBuilder;

impl CloudEventBuilder {
    /// Create a new `CloudEvent` in the current spec version
    pub fn default() -> DefaultCloudEventBuilder {
        DefaultCloudEventBuilder::default()
    }
    /// Create a new `CloudEvent` according to spec version 0.2
    pub fn v0_2() -> CloudEventV0_2Builder {
        CloudEventV0_2Builder::default()
    }
    /// Create a new `CloudEvent` according to spec version 1.0
    pub fn v1_0() -> CloudEventV1_0Builder {
        CloudEventV1_0Builder::default()
    }
}
