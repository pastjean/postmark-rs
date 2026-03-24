use std::borrow::Cow;

use crate::Endpoint;
use crate::api::templates::TemplateType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Validate template content.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct ValidateTemplateRequest {
    #[builder(default, setter(into, strip_option))]
    pub subject: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub html_body: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub text_body: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub test_render_model: Option<Value>,
    #[builder(default, setter(into, strip_option))]
    pub inline_css_for_html_test_render: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    pub template_type: Option<TemplateType>,
    #[builder(default, setter(into, strip_option))]
    pub layout_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidateTemplateResponse {
    pub all_content_is_valid: bool,
    pub html_body: ValidateTemplatePart,
    pub text_body: ValidateTemplatePart,
    pub subject: ValidateTemplatePart,
    pub suggested_template_model: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidateTemplatePart {
    pub content_is_valid: bool,
    pub validation_errors: Vec<ValidateTemplateError>,
    pub rendered_content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidateTemplateError {
    pub message: String,
    pub line: Option<isize>,
    pub character_position: Option<isize>,
}

impl Endpoint for ValidateTemplateRequest {
    type Request = ValidateTemplateRequest;
    type Response = ValidateTemplateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/templates/validate".into()
    }

    fn body(&self) -> &Self::Request {
        self
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
    pub async fn validate_template() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/templates/validate"))
                .respond_with(json_encoded(json!({
                    "AllContentIsValid": true,
                    "Subject": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "name_Value subjectHeadline_Value"
                    },
                    "HtmlBody": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "address_Value name_Value"
                    },
                    "TextBody": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "phone_Value name_Value"
                    },
                    "SuggestedTemplateModel": {
                        "userName": "bobby joe"
                    }
                }))),
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let req = ValidateTemplateRequest::builder()
            .subject("{{name}}")
            .html_body("<b>{{name}}</b>")
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should get a response and be able to json decode it");

        assert!(resp.all_content_is_valid);
        assert!(resp.subject.content_is_valid);
        assert_eq!(resp.subject.validation_errors.len(), 0);
    }
}
