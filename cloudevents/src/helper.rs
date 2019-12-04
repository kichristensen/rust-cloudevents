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

/// Construct a [`CloudEvent`] according to the latest spec version
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
/// use cloudevents::cloudevent;
/// use cloudevents::{Data, CloudEvent};
///
/// let event: CloudEvent = cloudevent!(
///   event_type: "test type",
///   source: "http://www.google.com",
///   event_id: "id",
///   datacontenttype: "application/json",
///   data: Data::from_string("\"test\""),
/// ).unwrap();
/// ```
///
/// [`CloudEvent`]: struct.CloudEvent.html
#[macro_export]
macro_rules! cloudevent {
    ($( $name:ident: $value:expr $(,)* )+) => {
        $crate::cloudevent_v1_0!($($name: $value,)*)
            .and_then(|event| Ok($crate::CloudEvent::V1_0(event)))
    };
}
