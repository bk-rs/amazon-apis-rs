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
#[serde(rename_all = "PascalCase")]
pub struct DetectLabelsRequestBody {
    pub image: Image,
    pub max_labels: Option<usize>,
    pub min_confidence: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DetectLabelsResponseOkBody {
    pub label_model_version: String,
    pub labels: Vec<Label>,
    pub orientation_correction: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_ok_body() {
        match serde_json::from_str::<DetectLabelsResponseOkBody>(include_str!(
            "../../../tests/response_body_json_files/detect_labels_ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.labels.len(), 13);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
