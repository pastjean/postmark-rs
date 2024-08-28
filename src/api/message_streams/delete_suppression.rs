use std::borrow::Cow;

use crate::api::message_streams::{StreamIdOrName, SuppressionStatusType};
use crate::Endpoint;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct DeleteSuppressionRequest {
    #[serde(skip)]
    pub stream_id: StreamIdOrName,
    pub suppressions: Vec<Emails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSuppressionResponse {
    pub suppressions: Vec<DeleteSuppression>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeleteSuppression {
    pub email_address: String,
    pub status: SuppressionStatusType,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Emails {
    pub email_address: String,
}

impl Endpoint for DeleteSuppressionRequest {
    type Request = DeleteSuppressionRequest;
    type Response = DeleteSuppressionResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/message-streams/{}/suppressions/delete", self.stream_id).into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::reqwest::PostmarkClient;
    use crate::Query;
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;

    const STREAM_ID: &str = "my-stream-id";

    const EMAIL_1: &str = "good.address@wildbit.com";
    const EMAIL_2: &str = "not.suppressed@wildbit.com";
    const EMAIL_3: &str = "spammy.address@wildbit.com";
    #[tokio::test]
    pub async fn create_new_server() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path(
                "POST",
                format!("/message-streams/{STREAM_ID}/suppressions/delete"),
            ))
            .respond_with(json_encoded(json!({
              "Suppressions":[
                {
                  "EmailAddress":EMAIL_1,
                  "Status":"Deleted",
                  "Message": null
                },
                {
                  "EmailAddress":EMAIL_2,
                  "Status":"Deleted",
                  "Message": null
                },
                {
                  "EmailAddress":EMAIL_3,
                  "Status":"Failed",
                  "Message": "You do not have the required authority to change this suppression."
                }
              ]
            }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeleteSuppressionRequest::builder()
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

        print!("{}\n", req.endpoint());

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
