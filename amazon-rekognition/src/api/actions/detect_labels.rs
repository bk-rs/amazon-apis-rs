//! https://docs.aws.amazon.com/rekognition/latest/dg/API_DetectLabels.html
//! https://docs.aws.amazon.com/rekognition/latest/dg/labels-detect-labels-image.html

use serde::{Deserialize, Serialize};

use crate::{
    api::data_types::{Image, Label},
    ServiceEndpoint,
};

use super::Action;

pub fn new(
    access_key_id: String,
    secret_access_key: String,
    service_endpoint: ServiceEndpoint,
    request_body: DetectLabelsRequestBody,
) -> Action<DetectLabelsRequestBody, DetectLabelsResponseOkBody> {
    Action::new(
        access_key_id,
        secret_access_key,
        service_endpoint,
        request_body,
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectLabelsRequestBody {
    #[serde(rename = "Image")]
    pub image: Image,
    #[serde(rename = "MaxLabels")]
    pub max_labels: Option<usize>,
    #[serde(rename = "MinConfidence")]
    pub min_confidence: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectLabelsResponseOkBody {
    #[serde(rename = "LabelModelVersion")]
    pub label_model_version: String,
    #[serde(rename = "Labels")]
    pub labels: Vec<Label>,
    #[serde(rename = "OrientationCorrection")]
    pub orientation_correction: Option<String>,
}
