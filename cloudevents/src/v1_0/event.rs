use crate::Data;
use crate::ExtensionValue;
use chrono::prelude::{DateTime, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

/// CloudEvent according to spec version 1.0
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CloudEventV1_0 {
    #[serde(rename = "type")]
    event_type: String,
    specversion: String,
    source: String,
    id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    dataschema: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    datacontenttype: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<HashMap<String, ExtensionValue>>,
}

impl CloudEventV1_0 {
    pub fn new(
        event_type: String,
        source: String,
        id: String,
        time: Option<DateTime<FixedOffset>>,
        subject: Option<String>,
        dataschema: Option<String>,
        datacontenttype: Option<String>,
        data: Option<Data>,
        extensions: Option<HashMap<String, ExtensionValue>>,
    ) -> Self {
        Self {
            event_type,
            specversion: String::from("1.0"),
            source,
            id,
            time,
            subject,
            dataschema,
            datacontenttype,
            data,
            extensions,
        }
    }
    /// Get the event type
    pub fn event_type(&self) -> &str {
        self.event_type.as_ref()
    }

    /// Get the source
    pub fn source(&self) -> &str {
        self.source.as_ref()
    }

    /// Get the event id
    pub fn event_id(&self) -> &str {
        self.id.as_ref()
    }

    /// Get the event time
    pub fn event_time(&self) -> Option<&DateTime<FixedOffset>> {
        self.time.as_ref()
    }

    /// Get the subject
    pub fn subject(&self) -> Option<&str> {
        self.subject.as_ref().map(|x| x.as_ref())
    }

    /// Get the dataschema
    pub fn dataschema(&self) -> Option<&str> {
        self.dataschema.as_ref().map(|x| x.as_ref())
    }

    /// Get the datacontenttype
    pub fn datacontenttype(&self) -> Option<&str> {
        self.datacontenttype.as_ref().map(|x| x.as_ref())
    }

    /// Get the data
    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    /// Get the extensions
    pub fn extensions(&self) -> Option<&HashMap<String, ExtensionValue>> {
        self.extensions.as_ref()
    }
}
