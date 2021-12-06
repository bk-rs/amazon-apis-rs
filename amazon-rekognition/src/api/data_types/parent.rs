//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Parent.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Parent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
