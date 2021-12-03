//! https://docs.aws.amazon.com/rekognition/latest/dg/API_DetectLabels.html

use serde::{Deserialize, Serialize};

use crate::{
    api::{
        data_types::{image::Image, label::Label},
        utils::{
            required_header_x_amz_target_value, REQUIRED_HEADER_CONTENT_TYPE_VALUE,
            REQUIRED_HEADER_X_AMZ_TARGET_KEY,
        },
    },
    ServiceEndpoint,
};

#[derive(Debug, Clone)]
pub struct DetectLabels {
    service_endpoint: ServiceEndpoint,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectLabelsRequestBody {
    #[serde(rename = "Image")]
    image: Image,
    #[serde(rename = "MaxLabels")]
    max_labels: Option<usize>,
    #[serde(rename = "MinConfidence")]
    min_confidence: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectLabelsResponseOkBody {
    #[serde(rename = "LabelModelVersion")]
    label_model_version: String,
    #[serde(rename = "Labels")]
    labels: Vec<Label>,
    #[serde(rename = "OrientationCorrection")]
    orientation_correction: Option<String>,
}
