//! https://docs.aws.amazon.com/rekognition/latest/dg/API_BoundingBox.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BoundingBox {
    pub height: Option<f64>,
    pub left: Option<f64>,
    pub top: Option<f64>,
    pub width: Option<f64>,
}
