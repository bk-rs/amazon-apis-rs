//! https://docs.aws.amazon.com/rekognition/latest/dg/API_DetectLabels.html
//! https://docs.aws.amazon.com/rekognition/latest/dg/labels-detect-labels-image.html

use serde::{Deserialize, Serialize};

use crate::{
    api::data_types::{Image, Label},
    ServiceEndpoint,
};

use super::Action;

pub const OPERATION: &str = "DetectLabels";

pub fn new<'a>(
    access_key_id: &'a str,
    secret_access_key: &'a str,
    service_endpoint: &'a ServiceEndpoint,
    request_body: DetectLabelsRequestBody,
) -> Action<'a, DetectLabelsRequestBody, DetectLabelsResponseOkBody> {
    Action::new(
        access_key_id,
        secret_access_key.as_ref(),
        service_endpoint,
        request_body,
        OPERATION,
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
