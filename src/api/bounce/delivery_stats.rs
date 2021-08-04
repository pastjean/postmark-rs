use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DeliveryStatsRequest {}

impl Endpoint for DeliveryStatsRequest {
    type Request = ();
    type Response = DeliveryStatsResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/deliverystats".into()
    }

    fn body(&self) -> &Self::Request {
        &()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]

pub struct DeliveryStatsResponse {
    #[serde(rename = "InactiveMails")]
    /// Number of inactive emails
    pub inactive_mails: i64,
    /// List of [bounce types](https://postmarkapp.com/developer/api/bounce-api#bounce-types) with total counts.
    pub bounces: Vec<Bounce>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Bounce {
    pub name: String,
    pub count: i64,
    pub type_field: Option<String>,
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::DeliveryStatsRequest;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    pub async fn send_get_deliverystats() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/deliverystats")).respond_with(
                json_encoded(json!({
                  "InactiveMails": 192,
                  "Bounces": [
                    {
                      "Name": "All",
                      "Count": 253
                    },
                    {
                      "Type": "HardBounce",
                      "Name": "Hard bounce",
                      "Count": 195
                    },
                    {
                      "Type": "Transient",
                      "Name": "Message delayed",
                      "Count": 10
                    },
                    {
                      "Type": "AutoResponder",
                      "Name": "Auto responder",
                      "Count": 14
                    },
                    {
                      "Type": "SpamNotification",
                      "Name": "Spam notification",
                      "Count": 3
                    },
                    {
                      "Type": "SoftBounce",
                      "Name": "Soft bounce",
                      "Count": 30
                    },
                    {
                      "Type": "SpamComplaint",
                      "Name": "Spam complaint",
                      "Count": 1
                    }
                  ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = DeliveryStatsRequest::default();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.inactive_mails, 192);
    }
}
