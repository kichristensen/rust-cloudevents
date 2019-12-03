/// Construct a [`CloudEvent`].
///
/// # Errors
///
/// If some of the required fields are missing, or if some of the fields
/// have invalid content an error is returned.
///
/// # Example
///
/// ```
/// #[macro_use]
/// use cloudevents::cloudevent_v1_0;
/// use cloudevents::Data;
///
/// let event = cloudevent_v1_0!(
///   event_type: "test type",
///   source: "http://www.google.com",
///   event_id: "id",
///   datacontenttype: "application/json",
///   data: Data::from_string("\"test\""),
/// ).unwrap();
/// ```
///
/// ## Date now
///
/// ```
/// #[macro_use]
/// use cloudevents::{cloudevent_v1_0, Data};
///
/// let event = cloudevent_v1_0!(
///   event_type: "test type",
///   source: "http://www.google.com",
///   event_id: "id",
///   datacontenttype: "application/json",
///   time: "now",
///   data: Data::from_string("\"test\""),
/// ).unwrap();
/// ```
///
/// [ `CloudEvent`]: struct.CloudEvent.html
#[macro_export]
macro_rules! cloudevent_v1_0 {
    ($( $name:ident: $value:expr $(,)* )+) => {
        $crate::v1_0::CloudEventBuilder::default()
            $(
                .$name($value)
            )*
            .build()
    };
}
