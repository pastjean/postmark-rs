use std::borrow::Cow;

use crate::Endpoint;
use crate::api::message_streams::{Emails, StreamIdOrName, SuppressionCreateStatusType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSuppressionRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
    pub suppressions: Vec<Emails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSuppressionResponse {
    pub suppressions: Vec<CreateSuppression>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateSuppression {
    pub email_address: String,
    pub status: SuppressionCreateStatusType,
    pub message: Option<String>,
}

impl Endpoint for CreateSuppressionRequest {
    type Request = CreateSuppressionRequest;
    type Response = CreateSuppressionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}/suppressions", self.stream_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::Query;
    use crate::reqwest::PostmarkClient;
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use super::*;

    const STREAM_ID: &str = "my-stream-id";

    const EMAIL_1: &str = "good.address@wildbit.com";
    const EMAIL_2: &str = "spammy.address@wildbit.com";
    const EMAIL_3: &str = "invalid-email-address";

    #[tokio::test]
    pub async fn create_suppression() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/message-streams/{STREAM_ID}/suppressions"),
            ))
            .respond_with(json_encoded(json!({
              "Suppressions":[
                {
                  "EmailAddress": EMAIL_1,
                  "Status": "Suppressed",
                  "Message": null
                },
                {
                  "EmailAddress": EMAIL_2,
                  "Status": "Failed",
                  "Message": "You do not have the required authority to change this suppression."
                },
                {
                  "EmailAddress": EMAIL_3,
                  "Status": "Failed",
                  "Message": "An invalid email address was provided."
                }
              ]
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CreateSuppressionRequest::builder()
            .stream_id(StreamIdOrName::StreamId(String::from(STREAM_ID)))
            .suppressions(Vec::from([
                Emails {
                    email_address: String::from(EMAIL_1),
                },
                Emails {
                    email_address: String::from(EMAIL_2),
                },
                Emails {
                    email_address: String::from(EMAIL_3),
                },
            ]))
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.suppressions.len(), 3);
        assert_eq!(
            resp.suppressions[0].status,
            SuppressionCreateStatusType::Suppressed
        );
        assert_eq!(
            resp.suppressions[1].status,
            SuppressionCreateStatusType::Failed
        );
    }
}
