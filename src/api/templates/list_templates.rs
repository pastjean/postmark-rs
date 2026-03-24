use crate::Endpoint;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use url::form_urlencoded::Serializer;

use super::TemplateType;

#[derive(Debug, Clone, PartialEq, Serialize, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[serde(rename_all = "PascalCase")]
pub struct ListTemplatesRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub template_type: Option<TemplateType>,
    pub layout_template: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListTemplatesResponse {
    pub total_count: isize,
    pub templates: Vec<TemplateSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TemplateSummary {
    pub template_id: isize,
    pub name: String,
    pub active: bool,
    pub alias: Option<String>,
    pub template_type: TemplateType,
    pub layout_template: Option<String>,
}

impl Endpoint for ListTemplatesRequest {
    type Request = ListTemplatesRequest;
    type Response = ListTemplatesResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        let mut serializer = Serializer::new(String::new());

        if let Some(count) = self.count {
            serializer.append_pair("count", &count.to_string());
        }
        if let Some(offset) = self.offset {
            serializer.append_pair("offset", &offset.to_string());
        }
        if let Some(ref template_type) = self.template_type {
            let template_type = match template_type {
                TemplateType::Standard => "Standard",
                TemplateType::Layout => "Layout",
            };
            serializer.append_pair("templateType", template_type);
        }
        if let Some(layout_template) = self.layout_template {
            serializer.append_pair("layoutTemplate", &layout_template.to_string());
        }

        let query = serializer.finish();
        if query.is_empty() {
            "/templates".into()
        } else {
            format!("/templates?{query}").into()
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
    use httptest::{responders::*, Expectation, Server};
    use serde_json::json;

    use super::*;
    use crate::reqwest::PostmarkClient;
    use crate::Query;

    #[tokio::test]
    async fn list_templates_uses_get_templates_path_and_decodes_response() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/templates")).respond_with(
                json_encoded(json!({
                    "TotalCount": 1,
                    "Templates": [
                        {
                            "TemplateId": 12345,
                            "Name": "Onboarding",
                            "Active": true,
                            "Alias": "onboarding",
                            "TemplateType": "Standard",
                            "LayoutTemplate": "base-layout"
                        }
                    ]
                })),
            ),
        );

        let req = ListTemplatesRequest::default();
        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/templates");

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode list templates response");

        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.templates[0].template_id, 12345);
        assert_eq!(resp.templates[0].template_type, TemplateType::Standard);
    }

    #[test]
    fn list_templates_with_filters_sets_query_path() {
        let req = ListTemplatesRequest {
            count: Some(10),
            offset: Some(20),
            template_type: Some(TemplateType::Layout),
            layout_template: Some(true),
        };

        let endpoint = req.endpoint();
        let endpoint = endpoint.as_ref();

        assert!(endpoint.starts_with("/templates?"));
        assert!(endpoint.contains("count=10"));
        assert!(endpoint.contains("offset=20"));
        assert!(endpoint.contains("templateType=Layout"));
        assert!(endpoint.contains("layoutTemplate=true"));
    }
}
