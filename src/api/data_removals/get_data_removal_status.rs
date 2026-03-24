use std::borrow::Cow;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::api::data_removals::DataRemovalResponse;
use crate::Endpoint;

#[derive(Debug, Clone, PartialEq, Serialize, TypedBuilder)]
pub struct GetDataRemovalStatusRequest {
    #[serde(skip)]
    #[builder(setter(into))]
    pub id: String,
}

impl Endpoint for GetDataRemovalStatusRequest {
    type Request = GetDataRemovalStatusRequest;
    type Response = DataRemovalResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("/data-removals/{}", self.id).into()
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
    use super::*;

    #[test]
    fn get_data_removal_status_uses_get_path() {
        let req = GetDataRemovalStatusRequest::builder().id("job-123").build();

        assert_eq!(req.method(), http::Method::GET);
        assert_eq!(req.endpoint(), "/data-removals/job-123");
    }
}
