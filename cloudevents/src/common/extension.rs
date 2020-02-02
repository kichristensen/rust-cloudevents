use failure::Error;
use serde::ser::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(untagged)]
/// Possible extension values
pub enum ExtensionValue {
    /// Represents a [`String`] value.
    ///
    /// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
    String(String),
    /// Represents a JSON [`Value`].
    ///
    /// [`Value`]: https://docs.serde.rs/serde_json/value/enum.Value.html
    Object(Value),
}

impl ExtensionValue {
    /// Create an [`ExtensionValue`] from a [`Into<String>`].
    ///
    /// # Example
    ///
    /// ```
    /// use cloudevents::ExtensionValue;
    ///
    /// let value = ExtensionValue::from_string("value");
    /// assert_eq!(value, ExtensionValue::String("value".to_owned()));
    /// ```
    ///
    /// [`Into<String>`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    /// [`ExtensionValue`]: enum.ExtensionValue.html
    pub fn from_string<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        ExtensionValue::String(s.into())
    }

    /// Create an [`ExtensionValue`] from a [`Serialize`] object.
    ///
    /// # Example
    ///
    /// ```
    /// use cloudevents::ExtensionValue;
    /// use serde_json::Value;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<Error>> {
    ///     let value = ExtensionValue::from_serializable("value")?;
    ///     assert_eq!(value, ExtensionValue::Object(Value::String("value".to_owned())));
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`Serialize`]: https://docs.serde.rs/serde/ser/trait.Serialize.html
    /// [`ExtensionValue`]: enum.ExtensionValue.html
    pub fn from_serializable<S>(s: S) -> Result<Self, Error>
    where
        S: Serialize,
    {
        Ok(ExtensionValue::Object(serde_json::to_value(s)?))
    }
}
