//! You'll find in `api` all the different predefined endpoints organized
//! by Postmark api sections.
//! 
//! In addition, some structures that are common to multiple endpoint API 
//! sections are included in here, specifically the definition of text 
//! and HTML based bodies. 

use serde::{Deserialize, Serialize};

pub mod bounce;
pub mod email;
pub mod templates;

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
