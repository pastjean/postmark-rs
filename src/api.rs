//! You'll find in `api` all the different predefined endpoints organized
//! by Postmark api sections.
//!
//! In addition, some structures that are common to multiple endpoint API
//! sections are included in here, specifically the definition of text
//! and HTML based bodies.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use url::Url;

pub mod bounce;
pub mod bulk;
pub mod data_removal;
pub mod domains;
pub mod email;
pub mod message_streams;
pub mod messages;
pub mod server;
pub mod signatures;
pub mod stats;
pub mod templates;
pub mod triggers;
pub mod types;
pub mod webhooks;

pub(crate) const DEFAULT_PAGE_COUNT: i64 = 100;
pub(crate) const DEFAULT_PAGE_OFFSET: i64 = 0;

pub(crate) fn endpoint_with_query(path: &str, query: String) -> Cow<'static, str> {
    let mut url = Url::parse("https://postmark.local").expect("valid static URL");
    {
        let mut segments = url.path_segments_mut().expect("URL supports path segments");
        for part in path.trim_start_matches('/').split('/') {
            if !part.is_empty() {
                segments.push(part);
            }
        }
    }

    if !query.is_empty() {
        url.set_query(Some(&query));
    }

    let mut endpoint = url.path().to_string();
    if let Some(query) = url.query() {
        endpoint.push('?');
        endpoint.push_str(query);
    }

    endpoint.into()
}

pub(crate) fn endpoint_with_path_segment(
    path_prefix: &str,
    path_segment: &str,
) -> Cow<'static, str> {
    let mut url = Url::parse("https://postmark.local").expect("valid static URL");
    {
        let mut segments = url.path_segments_mut().expect("URL supports path segments");
        for part in path_prefix.trim_start_matches('/').split('/') {
            if !part.is_empty() {
                segments.push(part);
            }
        }
        segments.push(path_segment);
    }
    url.path().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_with_query_uses_url_query_encoding() {
        let endpoint = endpoint_with_query(
            "messages/outbound",
            "recipient=user%40example.com&tag=one+two".to_string(),
        );

        assert_eq!(
            endpoint.as_ref(),
            "/messages/outbound?recipient=user%40example.com&tag=one+two"
        );
    }
}

/// The body of a email message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    Text {
        #[serde(rename = "TextBody")]
        text: String,
    },
    Html {
        #[serde(rename = "HtmlBody")]
        html: String,
    },
    HtmlAndText {
        #[serde(rename = "HtmlBody")]
        html: String,
        #[serde(rename = "TextBody")]
        text: String,
    },
}

impl Default for Body {
    fn default() -> Self {
        Body::Text { text: "".into() }
    }
}

impl Body {
    /// Constructor to create a text-only [`Body`] enum
    pub fn text(text: String) -> Self {
        Body::Text { text }
    }
    /// Constructor to create a html-only [`Body`] enum
    pub fn html(html: String) -> Self {
        Body::Html { html }
    }
    /// Constructor to create a text and html [`Body`] enum
    pub fn html_and_text(html: String, text: String) -> Self {
        Body::HtmlAndText { html, text }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlAndText {
    #[serde(flatten, rename = "HtmlBody")]
    pub html: String,
    #[serde(flatten, rename = "TextBody")]
    pub text: String,
}
