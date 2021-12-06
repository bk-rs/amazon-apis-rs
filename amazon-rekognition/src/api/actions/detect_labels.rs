//! https://docs.aws.amazon.com/rekognition/latest/dg/API_DetectLabels.html
//! https://docs.aws.amazon.com/rekognition/latest/dg/labels-detect-labels-image.html
//! https://stackoverflow.com/questions/48527517/an-example-of-calling-aws-rekognition-http-api-from-php

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Method,
    },
    Body, Endpoint, Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        actions::common::{action_parse_response, sign_request, EndpointError, EndpointRet},
        data_types::{Image, Label},
        utils::{
            required_header_x_amz_target_value, REQUIRED_HEADER_CONTENT_TYPE_VALUE,
            REQUIRED_HEADER_X_AMZ_TARGET_KEY,
        },
    },
    ServiceEndpoint,
};

#[derive(Debug, Clone)]
pub struct DetectLabels {
    access_key_id: String,
    secret_access_key: String,
    service_endpoint: ServiceEndpoint,
    request_body: DetectLabelsRequestBody,
}
impl DetectLabels {
    pub fn new(
        access_key_id: String,
        secret_access_key: String,
        service_endpoint: ServiceEndpoint,
        request_body: DetectLabelsRequestBody,
    ) -> Self {
        Self {
            access_key_id,
            secret_access_key,
            service_endpoint,
            request_body,
        }
    }
}

impl Endpoint for DetectLabels {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput = EndpointRet<DetectLabelsResponseOkBody>;

    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = self.service_endpoint.url();

        let body =
            serde_json::to_vec(&self.request_body).map_err(EndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(CONTENT_TYPE, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(ACCEPT, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(
                REQUIRED_HEADER_X_AMZ_TARGET_KEY,
                required_header_x_amz_target_value("DetectLabels"),
            )
            .body(body)
            .map_err(EndpointError::MakeRequestFailed)?;

        let request = sign_request(
            request,
            &self.access_key_id,
            &self.secret_access_key,
            &self.service_endpoint,
        )?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        action_parse_response(response)
    }
}

//
//
//
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
