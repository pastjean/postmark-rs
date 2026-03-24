use crate::{api::Body, Endpoint};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use typed_builder::TypedBuilder;

use super::TemplateType;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
#[derive(TypedBuilder)]
pub struct ValidateTemplateRequest {
    #[serde(flatten)]
    pub body: Body,

    #[builder(default, setter(into, strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub test_render_model: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub inline_css_for_html_test_render: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub template_type: Option<TemplateType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub layout_template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidateTemplateResponse {
    pub all_content_is_valid: bool,
    pub html_body: Option<ValidationResult>,
    pub text_body: Option<ValidationResult>,
    pub subject: Option<ValidationResult>,
    pub suggested_template_model: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidationResult {
    pub content_is_valid: bool,
    pub validation_errors: Vec<ValidationError>,
    pub rendered_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValidationError {
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

    fn method(&self) -> http::Method {
        http::Method::POST
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

    const HTML_BODY: &str = "<html><body>Hello {{name}}</body></html>";
    const TEXT_BODY: &str = "Hello {{name}}";
    const SUBJECT: &str = "Welcome {{name}}";

    #[tokio::test]
    async fn validate_template_uses_post_validate_path_serializes_and_decodes() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("POST", "/templates/validate"))
                .respond_with(json_encoded(json!({
                    "AllContentIsValid": true,
                    "HtmlBody": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "<html><body>Hello Ada</body></html>"
                    },
                    "TextBody": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "Hello Ada"
                    },
                    "Subject": {
                        "ContentIsValid": true,
                        "ValidationErrors": [],
                        "RenderedContent": "Welcome Ada"
                    },
                    "SuggestedTemplateModel": {
                        "name": "..."
                    }
                }))),
        );

        let req = ValidateTemplateRequest::builder()
            .body(Body::html_and_text(HTML_BODY.into(), TEXT_BODY.into()))
            .subject(SUBJECT)
            .test_render_model(json!({ "name": "Ada" }))
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/templates/validate");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({
                "HtmlBody": HTML_BODY,
                "TextBody": TEXT_BODY,
                "Subject": SUBJECT,
                "TestRenderModel": {
                    "name": "Ada"
                }
            })
        );

        let client = PostmarkClient::builder()
            .base_url(server.url("/").to_string())
            .build();

        let resp = req
            .execute(&client)
            .await
            .expect("Should decode validate template response");

        assert!(resp.all_content_is_valid);
        assert_eq!(
            resp.subject.unwrap().rendered_content.unwrap(),
            "Welcome Ada"
        );
    }

    #[test]
    fn validate_template_omits_subject_when_none() {
        let req = ValidateTemplateRequest::builder()
            .body(Body::html_and_text(HTML_BODY.into(), TEXT_BODY.into()))
            .build();

        let payload = serde_json::to_value(&req).unwrap();
        assert!(payload.get("Subject").is_none());
    }

    #[test]
    fn validate_template_serializes_optional_parity_fields() {
        let req = ValidateTemplateRequest::builder()
            .body(Body::html_and_text(HTML_BODY.into(), TEXT_BODY.into()))
            .inline_css_for_html_test_render(true)
            .template_type(TemplateType::Layout)
            .layout_template("base-layout")
            .build();

        let payload = serde_json::to_value(&req).unwrap();
        assert_eq!(payload["InlineCssForHtmlTestRender"], json!(true));
        assert_eq!(payload["TemplateType"], json!("Layout"));
        assert_eq!(payload["LayoutTemplate"], json!("base-layout"));
    }

    #[test]
    fn validate_template_omits_optional_parity_fields_when_none() {
        let req = ValidateTemplateRequest::builder()
            .body(Body::html_and_text(HTML_BODY.into(), TEXT_BODY.into()))
            .build();

        let payload = serde_json::to_value(&req).unwrap();
        assert!(payload.get("InlineCssForHtmlTestRender").is_none());
        assert!(payload.get("TemplateType").is_none());
        assert!(payload.get("LayoutTemplate").is_none());
    }
}
