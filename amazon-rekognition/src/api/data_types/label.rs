//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Label.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    #[serde(rename = "Confidence")]
    confidence: Option<f64>,
    // TODO
    #[serde(rename = "Name")]
    name: Option<String>,
    // TODO
}
