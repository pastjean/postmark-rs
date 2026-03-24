use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

use crate::api::messages::MessageSummary;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct InboundSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl Default for InboundSearchRequest {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InboundSearchResponse {
    pub total_count: i64,
    #[serde(alias = "InboundMessages")]
    pub messages: Vec<MessageSummary>,
}

impl Endpoint for InboundSearchRequest {
    type Request = InboundSearchRequest;
    type Response = InboundSearchResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }
        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }
        if let Some(ref mailbox_hash) = self.mailbox_hash {
            serializer.append_pair("mailboxhash", mailbox_hash);
        }
        if let Some(ref subject) = self.subject {
            serializer.append_pair("subject", subject);
        }
        if let Some(ref to) = self.to {
            serializer.append_pair("to", to);
        }

        let query = serializer.finish();
        if query.is_empty() {
            "/messages/inbound".into()
        } else {
            format!("/messages/inbound?{query}").into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn inbound_search_gets_messages() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/messages/inbound")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Messages": [
                        {
                            "MessageID": "in-msg-1",
                            "Subject": "Reply",
                            "Status": "Received",
                            "Tag": null,
                            "Recipient": "inbound@example.com"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = InboundSearchRequest::default();

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode inbound search");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.messages[0].message_id, "in-msg-1");
    }

    #[test]
    fn inbound_search_accepts_inbound_messages_alias() {
        let payload = json!({
            "TotalCount": 1,
            "InboundMessages": [
                {
                    "MessageID": "in-msg-alias-1"
                }
            ]
        });

        let resp: InboundSearchResponse =
            serde_json::from_value(payload).expect("Should decode inbound search alias");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.messages[0].message_id, "in-msg-alias-1");
    }

    #[test]
    fn inbound_search_encodes_query_params() {
        let req = InboundSearchRequest::builder()
            .count(5)
            .offset(15)
            .mailbox_hash("box-1".to_string())
            .subject("Reply".to_string())
            .to("inbound@example.com".to_string())
            .build();

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/messages/inbound?"));
        assert!(endpoint.contains("count=5"));
        assert!(endpoint.contains("offset=15"));
        assert!(endpoint.contains("mailboxhash=box-1"));
        assert!(endpoint.contains("subject=Reply"));
        assert!(endpoint.contains("to=inbound%40example.com"));
    }
}
