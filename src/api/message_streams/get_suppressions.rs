use std::borrow::Cow;

use crate::api::message_streams::StreamIdOrName;
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct GetSuppressionRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSuppressionResponse {
    pub suppressions: Vec<GetSuppression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetSuppression {
    pub email_address: String,
    pub suppression_reason: String,
    pub origin: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl Endpoint for GetSuppressionRequest {
    type Request = GetSuppressionRequest;
    type Response = GetSuppressionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}/suppressions/dump", self.stream_id).into()
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

    use crate::reqwest::PostmarkClient;
    use crate::Query;

    use super::*;
    const STREAM_ID: &str = "my-stream-id";
    #[tokio::test]
    pub async fn create_new_server() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "GET",
                format!("/message-streams/{STREAM_ID}/suppressions/dump"),
            ))
            .respond_with(json_encoded(json!({
              "Suppressions":[
                {
                  "EmailAddress":"address@wildbit.com",
                  "SuppressionReason":"ManualSuppression",
                  "Origin": "Recipient",
                  "CreatedAt":"2019-12-17T08:58:33-05:00"
                },
                {
                  "EmailAddress":"bounce.address@wildbit.com",
                  "SuppressionReason":"HardBounce",
                  "Origin": "Recipient",
                  "CreatedAt":"2019-12-17T08:58:33-05:00"
                },
                {
                  "EmailAddress":"spam.complaint.address@wildbit.com",
                  "SuppressionReason":"SpamComplaint",
                  "Origin": "Recipient",
                  "CreatedAt":"2019-12-17T08:58:33-05:00"
                }
              ]
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = GetSuppressionRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .build();

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
