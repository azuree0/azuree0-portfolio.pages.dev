use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Repo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub language: Option<String>,
    #[serde(default)]
    pub stargazers_count: u32,
    pub updated_at: String,
    /// Optional screenshot path (static fallback only)
    #[serde(skip_deserializing, skip_serializing_if = "Option::is_none", default)]
    pub screenshot: Option<String>,
}
