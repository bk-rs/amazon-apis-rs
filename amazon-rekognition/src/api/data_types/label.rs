//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Label.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    #[serde(rename = "Confidence")]
    pub confidence: Option<f64>,
    // TODO
    #[serde(rename = "Name")]
    pub name: Option<String>,
    // TODO
}
