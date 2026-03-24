use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::data_removals::DataRemovalResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "PascalCase")]
pub struct CreateDataRemovalRequest {
    #[builder(setter(into))]
    pub email_address: String,
}

impl Endpoint for CreateDataRemovalRequest {
    type Request = CreateDataRemovalRequest;
    type Response = DataRemovalResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "/data-removals".into()
    }

    fn body(&self) -> &Self::Request {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn create_data_removal_uses_post_path_and_pascal_case_body() {
        let req = CreateDataRemovalRequest::builder()
            .email_address("user@example.com")
            .build();

        assert_eq!(req.method(), http::Method::POST);
        assert_eq!(req.endpoint(), "/data-removals");
        assert_eq!(
            serde_json::to_value(&req).unwrap(),
            json!({ "EmailAddress": "user@example.com" })
        );
    }
}
