use crate::Data;
use crate::ExtensionValue;
use chrono::prelude::{DateTime, FixedOffset};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

/// CloudEvent according to spec version 0.2
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CloudEventV0_2 {
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
    schemaurl: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    contenttype: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Data>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    extensions: Option<HashMap<String, ExtensionValue>>,
}

impl CloudEventV0_2 {
    pub fn new(
        event_type: String,
        source: String,
        id: String,
        time: Option<DateTime<FixedOffset>>,
        schemaurl: Option<String>,
        contenttype: Option<String>,
        data: Option<Data>,
        extensions: Option<HashMap<String, ExtensionValue>>,
    ) -> Self {
        CloudEventV0_2 {
            event_type,
            specversion: String::from("0.2"),
            source,
            id,
            time,
            schemaurl,
            contenttype,
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

    /// Get the schemaurl
    pub fn schema_url(&self) -> Option<&str> {
        self.schemaurl.as_ref().map(|x| x.as_ref())
    }

    /// Get the data
    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    /// Get the datacontenttype
    pub fn contenttype(&self) -> Option<&str> {
        self.contenttype.as_ref().map(|x| x.as_ref())
    }

    /// Get the extensions
    pub fn extensions(&self) -> Option<&HashMap<String, ExtensionValue>> {
        self.extensions.as_ref()
    }
}
