//! https://docs.aws.amazon.com/rekognition/latest/dg/API_DetectLabels.html
//! https://docs.aws.amazon.com/rekognition/latest/dg/labels-detect-labels-image.html
//! https://stackoverflow.com/questions/48527517/an-example-of-calling-aws-rekognition-http-api-from-php

use std::time::SystemTime;

use aws_sigv4::{
    http_request::{
        sign, Error as AwsSigv4HttpRequestSignError, SignableRequest, SigningParams,
        SigningSettings,
    },
    signing_params::BuildError as AwsSigv4SigningParamsBuildError,
};
use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError, Method, StatusCode,
    },
    Body, Endpoint, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeJsonError, Map, Value};

use crate::{
    api::{
        data_types::{Image, Label},
        utils::{
            required_header_x_amz_target_value, REQUIRED_HEADER_CONTENT_TYPE_VALUE,
            REQUIRED_HEADER_X_AMZ_TARGET_KEY,
        },
    },
    ServiceEndpoint, SERVICE_NAME,
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
    type RenderRequestError = DetectLabelsError;

    type ParseResponseOutput = Result<DetectLabelsResponseOkBody, (StatusCode, Map<String, Value>)>;

    type ParseResponseError = DetectLabelsError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = self.service_endpoint.url();

        let body = serde_json::to_vec(&self.request_body)
            .map_err(DetectLabelsError::SerRequestBodyFailed)?;

        let mut request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(CONTENT_TYPE, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(ACCEPT, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(
                REQUIRED_HEADER_X_AMZ_TARGET_KEY,
                required_header_x_amz_target_value("DetectLabels"),
            )
            .body(body)
            .map_err(DetectLabelsError::MakeRequestFailed)?;

        let signing_settings = SigningSettings::default();
        let signing_params = SigningParams::builder()
            .access_key(&self.access_key_id)
            .secret_key(&self.secret_access_key)
            .region(self.service_endpoint.region())
            .service_name(SERVICE_NAME)
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .map_err(DetectLabelsError::MakeSigningParamsFailed)?;

        let signable_request = SignableRequest::from(&request);
        let (signing_instructions, _signature) = sign(signable_request, &signing_params)
            .map_err(DetectLabelsError::SignFailed)?
            .into_parts();

        signing_instructions.apply_to_request(&mut request);

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        println!("{:?}", String::from_utf8(response.body().to_owned()));

        let status = response.status();
        match status {
            StatusCode::OK => Ok(Ok(serde_json::from_slice(response.body())
                .map_err(DetectLabelsError::DeResponseOkBodyFailed)?)),
            status => Ok(Err(serde_json::from_slice::<Map<String, Value>>(
                response.body(),
            )
            .map(|x| (status, x))
            .map_err(DetectLabelsError::DeResponseOkBodyFailed)?)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DetectLabelsError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("MakeSigningParamsFailed {0}")]
    MakeSigningParamsFailed(AwsSigv4SigningParamsBuildError),
    #[error("SignFailed {0}")]
    SignFailed(AwsSigv4HttpRequestSignError),
    #[error("DeResponseOkBodyFailed {0}")]
    DeResponseOkBodyFailed(SerdeJsonError),
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
