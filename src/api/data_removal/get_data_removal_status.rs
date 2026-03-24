use std::borrow::Cow;

use crate::api::data_removal::DataRemovalStatusResponse;
use crate::Endpoint;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct GetDataRemovalStatusRequest {
    #[serde(skip)]
    pub data_removal_id: isize,
}

impl Endpoint for GetDataRemovalStatusRequest {
    type Request = GetDataRemovalStatusRequest;
    type Response = DataRemovalStatusResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/data-removals/{}", self.data_removal_id).into()
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

    #[tokio::test]
    async fn get_data_removal_status() {
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/data-removals/1234"))
                .respond_with(json_encoded(json!({"ID": 1234, "Status": "Done"}))),
        );
        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();
        let req = GetDataRemovalStatusRequest::builder()
            .data_removal_id(1234)
            .build();
        let resp = req.execute(&client).await.expect("json decode");
        assert_eq!(resp.status, "Done");
    }
}
