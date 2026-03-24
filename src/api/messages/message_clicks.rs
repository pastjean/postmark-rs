use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

use crate::Endpoint;
use crate::api::messages::MessageClick;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct MessageClicksRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_stream: Option<String>,
}

impl Default for MessageClicksRequest {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MessageClicksResponse {
    pub total_count: i64,
    pub clicks: Vec<MessageClick>,
}

impl Endpoint for MessageClicksRequest {
    type Request = MessageClicksRequest;
    type Response = MessageClicksResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }
        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }
        if let Some(ref recipient) = self.recipient {
            serializer.append_pair("recipient", recipient);
        }
        if let Some(ref tag) = self.tag {
            serializer.append_pair("tag", tag);
        }
        if let Some(ref message_stream) = self.message_stream {
            serializer.append_pair("messagestream", message_stream);
        }

        let query = serializer.finish();
        if query.is_empty() {
            "/messages/outbound/clicks".into()
        } else {
            format!("/messages/outbound/clicks?{query}").into()
        }
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use super::*;
    use crate::Query;
    use crate::reqwest::PostmarkClient;

    #[tokio::test]
    async fn message_clicks_gets_events() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/messages/outbound/clicks"))
                .respond_with(json_encoded(json!({
                    "TotalCount": 1,
                    "Clicks": [
                        {
                            "MessageID": "out-msg-1",
                            "OriginalLink": "https://example.com"
                        }
                    ]
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = MessageClicksRequest::default();

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode message clicks");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.clicks[0].message_id, "out-msg-1");
    }

    #[test]
    fn message_clicks_encodes_query_params() {
        let req = MessageClicksRequest::builder()
            .count(10)
            .offset(20)
            .recipient("user@example.com".to_string())
            .tag("welcome".to_string())
            .message_stream("outbound".to_string())
            .build();

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/messages/outbound/clicks?"));
        assert!(endpoint.contains("count=10"));
        assert!(endpoint.contains("offset=20"));
        assert!(endpoint.contains("recipient=user%40example.com"));
        assert!(endpoint.contains("tag=welcome"));
        assert!(endpoint.contains("messagestream=outbound"));
    }
}
