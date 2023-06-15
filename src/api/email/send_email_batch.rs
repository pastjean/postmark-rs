use std::borrow::Cow;

use super::send_email::{SendEmailRequest, SendEmailResponse};
use crate::{Endpoint};

/// Send multiple emails at once
pub type SendEmailBatchRequest = Vec<SendEmailRequest>;
/// Response for [`SendEmailBatchRequest`]
pub type SendEmailBatchResponse = Vec<SendEmailResponse>;

impl Endpoint for SendEmailBatchRequest {
    type Request = SendEmailBatchRequest;
    type Response = SendEmailBatchResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/email/batch".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use crate::api::email::*;
    use crate::api::Body;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    pub async fn send_email_test() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/email/batch")).respond_with(
                json_encoded(json!([{
                    "To": "receiver@example.com",
                    "SubmittedAt": "2014-02-17T07:25:01.4178645-05:00",
                    "MessageID": "0a129aee-e1cd-480d-b08d-4f48548ff48d",
                    "ErrorCode": 0,
                    "Message": "OK"
                },{
                    "ErrorCode": 406,
                    "Message": "You tried to send to a recipient that has been marked as inactive. Found inactive addresses: example@example.com. Inactive recipients are ones that have generated a hard bounce, a spam complaint, or a manual suppression."
                }])),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req_builder = SendEmailRequest::builder()
            .from("pa@example.com")
            .body(Body::text("hello matt".into()))
            .subject("hello");

        let req: SendEmailBatchRequest = vec![
            req_builder.clone().to("mathieu@example.com").build(),
            req_builder.to("pa@example.com").build(),
        ];

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
