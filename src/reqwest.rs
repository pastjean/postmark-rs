use crate::{Client, Endpoint, POSTMARK_API_URL};
use async_trait::async_trait;
use typed_builder::TypedBuilder;

/// A representation of the asynchronous Postmark API for a single user.
/// Separate users should use separate instances of this.
///
/// A reqwest based [`Client`]
///
/// ```
/// # use postmark::reqwest::PostmarkClient;
/// let client = PostmarkClient::default();
/// ```
///
/// Using the builder:
/// ```
/// # use postmark::reqwest::PostmarkClient;
/// let client = PostmarkClient::builder()
///   .base_url("https://api.postmarkapp.com/")
///   .token("<sometoken>")
///   .build();
/// ```
#[derive(TypedBuilder, Debug)]
pub struct PostmarkClient {
    #[builder(default, setter(into, strip_option))]
    pub token: Option<String>,
    #[builder(default=POSTMARK_API_URL.into(), setter(into))]
    pub base_url: String,
}

impl Default for PostmarkClient {
    fn default() -> Self {
        Self {
            base_url: POSTMARK_API_URL.into(),
            token: None,
        }
    }
}

#[async_trait]
impl Client for PostmarkClient {
    type Error = reqwest::Error;

    async fn execute<R: Endpoint + Send>(&self, req: R) -> Result<R::Response, Self::Error> {
        let client = reqwest::Client::builder().build()?;

        let mut r = client
            .post(format!("{}{}", self.base_url, req.endpoint()))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(req.body());

        if let Some(tok) = &self.token {
            r = r.header("X-Postmark-Server-Token", tok.clone())
        }

        let resp = r.send().await?.json::<R::Response>().await?;

        Ok(resp)
    }
}
