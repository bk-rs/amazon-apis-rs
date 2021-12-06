//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Instance.html

use serde::{Deserialize, Serialize};

use super::BoundingBox;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}
