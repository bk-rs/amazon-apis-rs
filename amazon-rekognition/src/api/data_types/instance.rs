//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Instance.html

use serde::{Deserialize, Serialize};

use super::BoundingBox;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub bounding_box: Option<BoundingBox>,
    pub confidence: Option<f64>,
}
