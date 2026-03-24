use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

use crate::Endpoint;
use crate::api::messages::MessageOpensResponse;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct SingleMessageOpensRequest {
    #[builder(setter(!strip_option))]
    #[serde(skip)]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl Endpoint for SingleMessageOpensRequest {
    type Request = SingleMessageOpensRequest;
    type Response = MessageOpensResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }
        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }

        let query = serializer.finish();
        if query.is_empty() {
            format!("/messages/outbound/opens/{}", self.message_id).into()
        } else {
            format!("/messages/outbound/opens/{}?{query}", self.message_id).into()
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
    async fn single_message_opens_gets_events() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                "/messages/outbound/opens/out-msg-1",
            ))
            .respond_with(json_encoded(json!({
                "TotalCount": 1,
                "Opens": [
                    {
                        "MessageID": "out-msg-1",
                        "Recipient": "user@example.com"
                    }
                ]
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = SingleMessageOpensRequest::builder()
            .message_id("out-msg-1".to_string())
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode single message opens");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.opens[0].message_id, "out-msg-1");
    }
}
