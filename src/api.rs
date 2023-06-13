//! You'll find in `api` all the different predefined endpoints organized
//! by Postmark api sections

use serde::{Deserialize, Serialize};

pub mod bounce;
pub mod email;
pub mod templates;

/// The body of a email message. 
/// 
/// The body may come either or both of two types, text and html.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Body {
    #[serde(rename = "TextBody")]
    Text(String),
    #[serde(rename = "HtmlBody")]
    Html(String),
    HtmlAndText(HtmlAndText),
}

impl Default for Body {
    fn default() -> Self {
        Body::Text("".into())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlAndText {
    #[serde(flatten, rename = "HtmlBody")]
    pub html: String,
    #[serde(flatten, rename = "TextBody")]
    pub text: String,
}