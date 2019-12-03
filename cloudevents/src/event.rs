use crate::v0_2::CloudEventV0_2;
use crate::v1_0::CloudEventV1_0;

pub enum CloudEvent {
    V0_2(CloudEventV0_2),
    V1_0(CloudEventV1_0),
}
