//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Label.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Label {
    pub confidence: Option<f64>,
    // TODO
    pub name: Option<String>,
    // TODO
}
