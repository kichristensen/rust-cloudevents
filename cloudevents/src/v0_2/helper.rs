/// Construct a [`CloudEvent`] according to spec version 0.2.
///
/// # Errors
///
/// If some of the required fields are missing, or if some of the fields
/// have invalid content an error is returned.
///
/// # Example
///
/// ```
/// use cloudevents::cloudevent_v0_2;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<Error>> {
///     let cloudevent = cloudevent_v0_2!(
///         event_type: "com.example.object.delete.v2",
///         source: "https://github.com/cloudevents/spec/pull/123",
///         event_id: "0e72b6bd-1341-46b5-9907-efde752682c4",
///         contenttype: "application/json"
///     )?;
///     Ok(())
/// }
///
/// ```
/// [`CloudEvent`]: struct.CloudEventV0_2.html
#[macro_export]
macro_rules! cloudevent_v0_2 {
    ($( $name:ident: $value:expr $(,)* )+) => {
        $crate::v0_2::CloudEventV0_2Builder::default()
            $(
                .$name($value)
            )*
            .build()
    };
}
