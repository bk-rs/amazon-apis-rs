use std::{fmt, time::SystemTime};

use aws_sigv4::{
    http_request::{
        sign, Error as AwsSigv4HttpRequestSignError, SignableRequest, SigningParams,
        SigningSettings,
    },
    signing_params::BuildError as AwsSigv4SigningParamsBuildError,
};
use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body, Request, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;

use crate::{ServiceEndpoint, SERVICE_NAME};

//
//
//
#[derive(Debug, Clone)]
pub enum EndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    Other((StatusCode, Result<ActionResponseErrBody, Body>)),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionResponseErrBody {
    #[serde(rename = "__type")]
    pub r#type: String,
    #[serde(rename = "Message")]
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointError {
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
pub fn sign_request(
    mut request: Request<Body>,
    access_key_id: &str,
    secret_access_key: &str,
    service_endpoint: &ServiceEndpoint,
) -> Result<Request<Body>, EndpointError> {
    let signing_settings = SigningSettings::default();
    let signing_params = SigningParams::builder()
        .access_key(access_key_id)
        .secret_key(secret_access_key)
        .region(service_endpoint.region())
        .service_name(SERVICE_NAME)
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()
        .map_err(EndpointError::MakeSigningParamsFailed)?;

    let signable_request = SignableRequest::from(&request);
    let (signing_instructions, _signature) = sign(signable_request, &signing_params)
        .map_err(EndpointError::SignFailed)?
        .into_parts();

    signing_instructions.apply_to_request(&mut request);

    Ok(request)
}

pub fn action_parse_response<T>(response: Response<Body>) -> Result<EndpointRet<T>, EndpointError>
where
    T: fmt::Debug + Clone + DeserializeOwned,
{
    let status = response.status();
    match status {
        StatusCode::OK => {
            let ok_json = serde_json::from_slice::<T>(&response.body())
                .map_err(EndpointError::DeResponseOkBodyFailed)?;

            Ok(EndpointRet::Ok(ok_json))
        }
        status => match serde_json::from_slice::<ActionResponseErrBody>(&response.body()) {
            Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
            Err(_) => Ok(EndpointRet::Other((
                status,
                Err(response.body().to_owned()),
            ))),
        },
    }
}
