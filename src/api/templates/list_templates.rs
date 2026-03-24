use std::borrow::Cow;

use crate::Endpoint;
use crate::api::templates::TemplateType;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// List templates with pagination.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct ListTemplatesRequest {
    /// Number of templates to return.
    #[serde(skip)]
    pub count: isize,
    /// Number of templates to skip.
    #[serde(skip)]
    pub offset: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTemplatesResponse {
    /// Total number of templates associated with current server.
    pub total_count: isize,
    /// Templates list.
    pub templates: Vec<TemplateSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateSummary {
    pub active: bool,
    pub template_id: isize,
    pub name: String,
    pub alias: Option<String>,
    pub template_type: TemplateType,
    pub layout_template: Option<String>,
}

impl Endpoint for ListTemplatesRequest {
    type Request = ListTemplatesRequest;
    type Response = ListTemplatesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/templates?count={}&offset={}", self.count, self.offset).into()
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

    use crate::Query;
    use crate::reqwest::PostmarkClient;

    use super::*;

    #[tokio::test]
    pub async fn list_templates() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/templates")).respond_with(
                json_encoded(json!({
                    "TotalCount": 2,
                    "Templates": [
                        {
                            "TemplateId": 1234,
                            "Name": "Password Recovery Email",
                            "Alias": "password-recovery",
                            "Active": true,
                            "TemplateType": "Standard",
                            "LayoutTemplate": "my-layout"
                        },
                        {
                            "TemplateId": 5678,
                            "Name": "Default Layout",
                            "Alias": "my-layout",
                            "Active": true,
                            "TemplateType": "Layout",
                            "LayoutTemplate": null
                        }
                    ]
                })),
            ),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ListTemplatesRequest::builder().count(100).offset(0).build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert_eq!(resp.total_count, 2);
        assert_eq!(resp.templates.len(), 2);
        assert_eq!(resp.templates[0].template_id, 1234);
        assert_eq!(
            resp.templates[0].alias.as_deref(),
            Some("password-recovery")
        );
    }
}
