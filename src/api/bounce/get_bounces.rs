use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct GetBouncesRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub r#type: Option<String>,
    pub inactive: Option<bool>,
    pub email_filter: Option<String>,
    pub tag: Option<String>,
    pub message_id: Option<String>,
    pub message_stream: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

impl Default for GetBouncesRequest {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBouncesResponse {
    pub total_count: i64,
    pub bounces: Vec<GetBouncesBounce>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetBouncesBounce {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "Type")]
    pub type_field: Option<String>,
    pub email: String,
}

impl Endpoint for GetBouncesRequest {
    type Request = GetBouncesRequest;
    type Response = GetBouncesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }
        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }
        if let Some(ref type_field) = self.r#type {
            serializer.append_pair("type", type_field);
        }
        if let Some(inactive) = self.inactive {
            serializer.append_pair("inactive", &inactive.to_string());
        }
        if let Some(ref email_filter) = self.email_filter {
            serializer.append_pair("emailFilter", email_filter);
        }
        if let Some(ref tag) = self.tag {
            serializer.append_pair("tag", tag);
        }
        if let Some(ref message_id) = self.message_id {
            serializer.append_pair("messageID", message_id);
        }
        if let Some(ref message_stream) = self.message_stream {
            serializer.append_pair("messagestream", message_stream);
        }
        if let Some(ref from_date) = self.from_date {
            serializer.append_pair("fromdate", from_date);
        }
        if let Some(ref to_date) = self.to_date {
            serializer.append_pair("todate", to_date);
        }

        let query = serializer.finish();
        if query.is_empty() {
            "/bounces".into()
        } else {
            format!("/bounces?{query}").into()
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
    async fn send_get_bounces() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/bounces")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Bounces": [
                        {
                            "ID": 777,
                            "Type": "HardBounce",
                            "Email": "bounce@example.com"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetBouncesRequest::default();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.bounces[0].id, 777);
    }

    #[tokio::test]
    async fn get_bounces_with_filters_sets_query_path() {
        let req = GetBouncesRequest::builder()
            .count(10)
            .offset(20)
            .r#type(String::from("HardBounce"))
            .inactive(true)
            .email_filter(String::from("bounce@example.com"))
            .tag(String::from("welcome"))
            .message_id(String::from("msg-1"))
            .message_stream(String::from("outbound"))
            .from_date(String::from("2024-01-01"))
            .to_date(String::from("2024-02-01"))
            .build();

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/bounces?"));
        assert!(endpoint.contains("count=10"));
        assert!(endpoint.contains("offset=20"));
        assert!(endpoint.contains("type=HardBounce"));
        assert!(endpoint.contains("inactive=true"));
        assert!(endpoint.contains("emailFilter=bounce%40example.com"));
        assert!(endpoint.contains("tag=welcome"));
        assert!(endpoint.contains("messageID=msg-1"));
        assert!(endpoint.contains("messagestream=outbound"));
        assert!(endpoint.contains("fromdate=2024-01-01"));
        assert!(endpoint.contains("todate=2024-02-01"));
    }
}
