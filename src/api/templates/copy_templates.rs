use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::Endpoint;
use crate::api::server::ServerId;
use crate::api::templates::{TemplateAction, TemplateId, TemplateType};

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct CopyTemplatesRequest {
    #[serde(rename = "SourceServerID")]
    #[builder(setter(into))]
    pub source_server_id: ServerId,
    #[serde(rename = "DestinationServerID")]
    #[builder(setter(into))]
    pub destination_server_id: ServerId,
    #[builder(default = true)]
    pub perform_changes: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CopyTemplatesResponse {
    pub total_count: i64,
    pub templates: Vec<Template>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Template {
    pub action: TemplateAction,
    pub template_id: TemplateId,
    pub alias: Option<String>,
    pub name: String,
    pub template_type: TemplateType,
}

impl Endpoint for CopyTemplatesRequest {
    type Request = CopyTemplatesRequest;
    type Response = CopyTemplatesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/templates/push".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }

    fn method(&self) -> http::Method {
        http::Method::PUT
    }
}

#[cfg(test)]
mod tests {
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use serde_json::json;

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn push_templates() {
        let server = Server::run();

        const SOURCE_SERVER: i64 = 12345;
        const DESTINATION_SERVER: i64 = 23456;

        server.expect(
            Expectation::matching(request::method_path("PUT", "/templates/push")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Templates": [
                        {
                            "Action": "Create",
                            "TemplateId": 7270,
                            "Alias": "comment-notification",
                            "Name": "Comment notification",
                            "TemplateType": "Standard"
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = CopyTemplatesRequest::builder()
            .source_server_id(SOURCE_SERVER)
            .destination_server_id(DESTINATION_SERVER)
            .build();

        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "SourceServerID": SOURCE_SERVER,
                "DestinationServerID": DESTINATION_SERVER,
                "PerformChanges": true,
            })
        );

        req.execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");
    }
}
