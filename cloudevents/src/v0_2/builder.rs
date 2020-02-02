use super::CloudEventV0_2;
use crate::Data;
use crate::ExtensionValue;
use chrono::prelude::DateTime;
use failure::{format_err, Error};
use std::collections::HashMap;
use url::{ParseError, Url};

/// Create a new [`CloudEvent`] according to spec version 0.2.
///
/// # Example
///
/// ```
/// use cloudevents::v0_2::{CloudEventV0_2, CloudEventV0_2Builder};
/// use failure::Error;
///
/// let event : Result<CloudEventV0_2, Error> = CloudEventV0_2Builder::default()
///     .event_id("id")
///     .source("http://www.google.com")
///     .event_type("test type")
///     .contenttype("application/json")
///     .build();
/// ```
///
/// [`CloudEvent`]: struct.CloudEventV0_2.html
#[derive(Debug)]
pub struct CloudEventV0_2Builder {
    event_type: Option<String>,
    source: Option<String>,
    id: Option<String>,
    time: Option<String>,
    schemaurl: Option<String>,
    contenttype: Option<String>,
    data: Option<Data>,
    extensions: Option<HashMap<String, ExtensionValue>>,
}

impl CloudEventV0_2Builder {
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

    /// Set the schemaurl.
    pub fn schemaurl<S: Into<String>>(mut self, s: S) -> Self {
        self.schemaurl = Some(s.into());
        self
    }

    /// Set the content type.
    pub fn contenttype<S: Into<String>>(mut self, s: S) -> Self {
        self.contenttype = Some(s.into());
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
    pub fn build(self) -> Result<CloudEventV0_2, Error> {
        Ok(CloudEventV0_2::new(
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
                if let Some(t) = self.time {
                    Some(DateTime::parse_from_rfc3339(&t)?)
                } else {
                    None
                }
            },
            {
                if let Some(x) = self.schemaurl {
                    let schemaurl = x;
                    match Url::parse(&schemaurl) {
                        Ok(_) | Err(ParseError::RelativeUrlWithoutBase) => Some(schemaurl),
                        Err(e) => return Err(format_err!("{}", e)),
                    }
                } else {
                    None
                }
            },
            self.contenttype,
            self.data,
            self.extensions,
        ))
    }
}

impl Default for CloudEventV0_2Builder {
    fn default() -> Self {
        CloudEventV0_2Builder {
            event_type: None,
            id: None,
            schemaurl: None,
            source: None,
            extensions: None,
            data: None,
            contenttype: None,
            time: None,
        }
    }
}
