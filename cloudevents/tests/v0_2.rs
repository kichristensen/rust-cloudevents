use cloudevents::cloudevent_v0_2;
use cloudevents::{CloudEventBuilder, Data, ExtensionValue};
use serde_derive::Serialize;
use serde_json::json;

#[test]
fn string_data_can_be_created_from_str() {
    let content = "string content";
    let data = Data::from_string(content);
    assert_eq!(data, Data::StringOrBinary(content.to_owned()));
}

#[test]
fn binary_data_can_be_created_from_slice() {
    let data = Data::from_binary(b"this is binary");
    assert_eq!(
        data,
        Data::StringOrBinary("dGhpcyBpcyBiaW5hcnk=".to_owned())
    )
}

#[test]
fn object_data_can_be_created_from_serializable() {
    #[derive(Serialize)]
    struct SerializableStruct {
        content: String,
    }

    let object = SerializableStruct {
        content: "content".to_owned(),
    };
    let data = Data::from_serializable(object).unwrap();
    let expected = json!({
        "content": "content",
    });
    assert_eq!(data, Data::Object(expected));
}

#[test]
fn extension_string_data_can_be_created_from_str() {
    let content = "string content";
    let data = ExtensionValue::from_string(content);
    assert_eq!(data, ExtensionValue::String(content.to_owned()));
}

#[test]
fn extension_object_data_can_be_created_from_serializable() {
    #[derive(Serialize)]
    struct SerializableStruct {
        content: String,
    }

    let object = SerializableStruct {
        content: "content".to_owned(),
    };
    let data = ExtensionValue::from_serializable(object).unwrap();
    let expected = json!({
        "content": "content",
    });
    assert_eq!(data, ExtensionValue::Object(expected));
}

#[test]
fn builder_works() {
    let event = CloudEventBuilder::v0_2()
        .event_id("id")
        .source("http://www.google.com")
        .event_type("test type")
        .contenttype("application/json")
        .build()
        .unwrap();

    assert_eq!(event.event_type(), "test type");
    assert_eq!(event.source(), "http://www.google.com");
    assert_eq!(event.event_id(), "id");
    assert_eq!(event.extensions(), None);
    assert_eq!(event.data(), None);
    assert_eq!(event.event_time(), None);
    assert_eq!(event.contenttype(), Some("application/json"));
    assert_eq!(event.schema_url(), None);
}

#[test]
fn builder_macro_works() {
    let event = cloudevent_v0_2!(
        event_type: "test type",
        source: "http://www.google.com",
        event_id: "id",
        contenttype: "application/json",
        data: Data::from_string("test"),
    )
    .unwrap();

    assert_eq!(event.event_type(), "test type");
    assert_eq!(event.source(), "http://www.google.com");
    assert_eq!(event.event_id(), "id");
    assert_eq!(event.extensions(), None);
    assert_eq!(event.data(), Some(&Data::StringOrBinary("test".to_owned())));
    assert_eq!(event.event_time(), None);
    assert_eq!(event.contenttype(), Some("application/json"));
    assert_eq!(event.schema_url(), None);
}

#[test]
fn source_is_allowed_to_be_a_relative_uri() {
    let event = CloudEventBuilder::v0_2()
        .event_id("id")
        .source("/cloudevents/spec/pull/123")
        .event_type("test type")
        .build()
        .unwrap();

    assert_eq!(event.source(), "/cloudevents/spec/pull/123");
}

#[test]
fn source_is_allowed_to_be_a_urn() {
    let event = CloudEventBuilder::v0_2()
        .event_id("id")
        .source("urn:event:from:myapi/resourse/123")
        .event_type("test type")
        .build()
        .unwrap();

    assert_eq!(event.source(), "urn:event:from:myapi/resourse/123");
}

#[test]
fn source_is_allowed_to_be_a_mailto() {
    let event = CloudEventBuilder::v0_2()
        .event_id("id")
        .source("mailto:cncf-wg-serverless@lists.cncf.io")
        .event_type("test type")
        .build()
        .unwrap();

    assert_eq!(event.source(), "mailto:cncf-wg-serverless@lists.cncf.io");
}

#[test]
fn serialize() {
    let event = cloudevent_v0_2!(
        event_type: "test type",
        source: "http://www.google.com",
        event_id: "id",
        contenttype: "application/json",
        data: Data::from_string("\"test\""),
    );

    let json = serde_json::to_string(&event.unwrap()).unwrap();
    assert_eq!(json, "{\"type\":\"test type\",\"specversion\":\"0.2\",\"source\":\"http://www.google.com\",\"id\":\"id\",\"contenttype\":\"application/json\",\"data\":\"\\\"test\\\"\"}");
}
