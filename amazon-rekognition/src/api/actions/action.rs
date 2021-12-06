//! https://stackoverflow.com/questions/48527517/an-example-of-calling-aws-rekognition-http-api-from-php

use std::{fmt, marker::PhantomData};

use amazon_sigv4_kit::{sign_http_request, SignHttpRequestError};
use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, CONTENT_TYPE},
        Error as HttpError, Method, StatusCode,
    },
    Body, Endpoint, Request, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

use crate::{
    api::utils::{
        required_header_x_amz_target_value, REQUIRED_HEADER_CONTENT_TYPE_VALUE,
        REQUIRED_HEADER_X_AMZ_TARGET_KEY,
    },
    ServiceEndpoint, SERVICE_NAME,
};

//
//
//
pub struct Action<'a, ReqB, ResOkB>
where
    ReqB: Serialize,
    ResOkB: DeserializeOwned + fmt::Debug + Clone,
{
    pub access_key_id: &'a str,
    pub secret_access_key: &'a str,
    pub service_endpoint: ServiceEndpoint<'a>,
    pub request_body: ReqB,
    pub operation_name: &'a str,
    //
    _phantom: PhantomData<ResOkB>,
}

impl<'a, ReqB, ResOkB> fmt::Debug for Action<'a, ReqB, ResOkB>
where
    ReqB: Serialize + fmt::Debug,
    ResOkB: DeserializeOwned + fmt::Debug + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Action")
            .field("access_key_id", &self.access_key_id)
            .field("secret_access_key", &format_args!("******"))
            .field("service_endpoint", &self.service_endpoint)
            .field("request_body", &self.request_body)
            .field("operation_name", &self.operation_name)
            .finish()
    }
}

impl<'a, ReqB, ResOkB> Clone for Action<'a, ReqB, ResOkB>
where
    ReqB: Serialize + Clone,
    ResOkB: DeserializeOwned + fmt::Debug + Clone,
{
    fn clone(&self) -> Self {
        Self {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
            service_endpoint: self.service_endpoint.clone(),
            request_body: self.request_body.clone(),
            operation_name: self.operation_name,
            _phantom: PhantomData,
        }
    }
}

impl<'a, ReqB, ResOkB> Action<'a, ReqB, ResOkB>
where
    ReqB: Serialize,
    ResOkB: DeserializeOwned + fmt::Debug + Clone,
{
    pub fn new(
        access_key_id: &'a str,
        secret_access_key: &'a str,
        service_endpoint: ServiceEndpoint<'a>,
        request_body: ReqB,
        operation_name: &'a str,
    ) -> Self {
        Self {
            access_key_id,
            secret_access_key,
            service_endpoint,
            request_body,
            operation_name,
            _phantom: PhantomData,
        }
    }
}

impl<'a, ReqB, ResOkB> Endpoint for Action<'a, ReqB, ResOkB>
where
    ReqB: Serialize,
    ResOkB: DeserializeOwned + fmt::Debug + Clone,
{
    type RenderRequestError = ActionEndpointError;

    type ParseResponseOutput = ActionEndpointRet<ResOkB>;

    type ParseResponseError = ActionEndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = self.service_endpoint.url();

        let body = serde_json::to_vec(&self.request_body)
            .map_err(ActionEndpointError::SerRequestBodyFailed)?;

        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header(CONTENT_TYPE, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(ACCEPT, REQUIRED_HEADER_CONTENT_TYPE_VALUE)
            .header(
                REQUIRED_HEADER_X_AMZ_TARGET_KEY,
                required_header_x_amz_target_value(self.operation_name),
            )
            .body(body)
            .map_err(ActionEndpointError::MakeRequestFailed)?;

        let request = sign_http_request(
            request,
            self.access_key_id,
            self.secret_access_key,
            self.service_endpoint.region(),
            SERVICE_NAME,
            |ss| ss,
            |sp| sp,
        )
        .map_err(ActionEndpointError::SignHttpRequestError)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => {
                let ok_json = serde_json::from_slice::<ResOkB>(response.body())
                    .map_err(ActionEndpointError::DeResponseOkBodyFailed)?;

                Ok(ActionEndpointRet::Ok(ok_json))
            }
            status => match serde_json::from_slice::<ActionResponseErrBody>(response.body()) {
                Ok(err_json) => Ok(ActionEndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(ActionEndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
#[derive(Debug, Clone)]
pub enum ActionEndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<ActionResponseErrBody, Body>)),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ActionResponseErrBody {
    #[serde(rename = "__type")]
    pub r#type: String,
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum ActionEndpointError {
    #[error("SerRequestBodyFailed {0}")]
    SerRequestBodyFailed(SerdeJsonError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("SignHttpRequestError {0}")]
    SignHttpRequestError(SignHttpRequestError),
    #[error("DeResponseOkBodyFailed {0}")]
    DeResponseOkBodyFailed(SerdeJsonError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de_response_err_body() {
        match serde_json::from_str::<ActionResponseErrBody>(include_str!(
            "../../../tests/response_body_json_files/detect_labels_err.json"
        )) {
            Ok(err_json) => {
                assert_eq!(err_json.r#type, "SerializationException");
            }
            Err(err) => panic!("{}", err),
        }
    }
}
