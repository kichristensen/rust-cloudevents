/// Get a attribute from a wrapped [`CloudEvent`] version.
///
/// # Example
///
/// ```
/// #[macro_use]
/// use cloudevents::get_event_field;
/// #[macro_use]
/// use cloudevents::cloudevent_v1_0;
/// use cloudevents::Data;
/// use cloudevents::CloudEvent;
///
/// let event = cloudevent_v1_0!(
///   event_type: "test type",
///   source: "http://www.google.com",
///   event_id: "id",
///   datacontenttype: "application/json",
///   data: Data::from_string("\"test\""),
/// ).unwrap();
/// let event = CloudEvent::V1_0(event);
///
/// let id = get_event_field!(event, event_id);
/// assert_eq!(id,"id");
/// ```
///
#[macro_export]
macro_rules! get_event_field {
    ($event:expr, $value:ident) => {
        match $event {
            $crate::CloudEvent::V0_2(ref e) => e.$value(),
            $crate::CloudEvent::V1_0(ref e) => e.$value(),
        }
    };
}
