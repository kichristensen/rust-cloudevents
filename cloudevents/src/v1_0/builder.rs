use super::CloudEventV1_0;
use crate::Data;
use crate::ExtensionValue;
use chrono::prelude::{DateTime, FixedOffset, Local};
use failure::{format_err, Error};
use std::collections::HashMap;
use url::{ParseError, Url};

/// Create a new [`CloudEvent`] according to spec version 0.2.
///
/// # Example
///
/// ```
/// use cloudevents::v1_0::{CloudEventV1_0,CloudEventV1_0Builder};
/// use failure::Error;
///
/// let event : Result<CloudEventV1_0, Error> = CloudEventV1_0Builder::default()
///     .event_id("id")
///     .source("http://www.google.com")
///     .event_type("test type")
///     .datacontenttype("application/json")
///     .build();
/// ```
///
/// [`CloudEvent`]: struct.CloudEventV1_0.html
#[derive(Debug)]
pub struct CloudEventV1_0Builder {
    event_type: Option<String>,
    source: Option<String>,
    id: Option<String>,
    time: Option<String>,
    subject: Option<String>,
    dataschema: Option<String>,
    datacontenttype: Option<String>,
    data: Option<Data>,
    extensions: Option<HashMap<String, ExtensionValue>>,
}

impl CloudEventV1_0Builder {
    /// Set the event type.
    pub fn event_type<S: Into<String>>(mut self, s: S) -> Self {
        self.event_type = Some(s.into());
        self
    }

    /// Set the source.
    pub fn source<S: Into<String>>(mut self, s: S) -> Self {
        self.source = Some(s.into());
        self
    }

    /// Set the event id.
    pub fn event_id<S: Into<String>>(mut self, s: S) -> Self {
        self.id = Some(s.into());
        self
    }

    /// Set the time.
    pub fn time<S: Into<String>>(mut self, s: S) -> Self {
        self.time = Some(s.into());
        self
    }

    /// Set the subject.
    pub fn subject<S: Into<String>>(mut self, s: S) -> Self {
        self.subject = Some(s.into());
        self
    }

    /// Set the dataschema.
    pub fn dataschema<S: Into<String>>(mut self, s: S) -> Self {
        self.dataschema = Some(s.into());
        self
    }

    /// Set the datacontenttype.
    pub fn datacontenttype<S: Into<String>>(mut self, s: S) -> Self {
        self.datacontenttype = Some(s.into());
        self
    }

    /// Set the data.
    pub fn data(mut self, d: Data) -> Self {
        self.data = Some(d);
        self
    }

    /// Set the extensions.
    pub fn extensions(mut self, e: HashMap<String, ExtensionValue>) -> Self {
        self.extensions = Some(e);
        self
    }

    /// Build a [`CloudEvent`].
    ///
    /// # Errors
    ///
    /// An error is thrown if one of the required fields (event_type, id or source) is not populated,
    /// or if one of the validated fields (time, source and schemeurl) are populated with an invalid value.
    ///
    /// [`CloudEvent`]: struct.CloudEvent.html
    pub fn build(self) -> Result<CloudEventV1_0, Error> {
        Ok(CloudEventV1_0::new(
            self.event_type
                .ok_or(format_err!("Event type is required"))?,
            {
                if let Some(x) = self.source {
                    let source = x;
                    match Url::parse(&source) {
                        Ok(_) | Err(ParseError::RelativeUrlWithoutBase) => source,
                        Err(e) => return Err(format_err!("{}", e)),
                    }
                } else {
                    return Err(format_err!("Source is required"));
                }
            },
            self.id.ok_or(format_err!("Event id is required"))?,
            {
                match self.time.as_ref() {
                    Some(t) if t == "now" => Some(DateTime::<FixedOffset>::from(Local::now())),
                    Some(t) => Some(DateTime::parse_from_rfc3339(&t)?),
                    None => None,
                }
            },
            self.subject,
            {
                match self.dataschema {
                    Some(dataschema) => match Url::parse(&dataschema) {
                        Ok(_) | Err(ParseError::RelativeUrlWithoutBase) => Some(dataschema),
                        Err(e) => return Err(format_err!("{}", e)),
                    },
                    None => None,
                }
            },
            self.datacontenttype,
            self.data,
            self.extensions,
        ))
    }
}

impl Default for CloudEventV1_0Builder {
    fn default() -> Self {
        CloudEventV1_0Builder {
            event_type: None,
            source: None,
            id: None,
            time: None,
            subject: None,
            dataschema: None,
            datacontenttype: None,
            data: None,
            extensions: None,
        }
    }
}
