use crate::v0_2::CloudEventV0_2;
use crate::v1_0::CloudEventV1_0;
use serde_derive::{Deserialize, Serialize};

/// Generic CloudEvent wrapping all spec versions
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CloudEvent {
    V1_0(CloudEventV1_0),
    V0_2(CloudEventV0_2),
}
