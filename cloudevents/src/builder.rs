use crate::v0_2::CloudEventV0_2Builder;
use crate::v1_0::CloudEventV1_0Builder;

type DefaultCloudEventBuilder = CloudEventV1_0Builder;

pub struct CloudEventBuilder;

impl CloudEventBuilder {
    pub fn default() -> DefaultCloudEventBuilder {
        DefaultCloudEventBuilder::default()
    }
    pub fn v0_2() -> CloudEventV0_2Builder {
        CloudEventV0_2Builder::default()
    }
    pub fn v1_0() -> CloudEventV1_0Builder {
        CloudEventV1_0Builder::default()
    }
}
