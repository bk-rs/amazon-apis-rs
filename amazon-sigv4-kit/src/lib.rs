use std::{
    error, fmt,
    time::{Duration, SystemTime},
};

use aws_sigv4::{
    http_request::{
        sign, Error as AwsSigv4HttpRequestSignError, SignableRequest, SigningParams,
        SigningSettings,
    },
    signing_params::BuildError as AwsSigv4SigningParamsBuildError,
};
use http::Request;

//
pub fn sign_http_request<SSCB, SRCB>(
    mut request: Request<Vec<u8>>,
    access_key_id: &str,
    secret_access_key: &str,
    region: &str,
    service_name: &str,
    mut signing_settings_callback: SSCB,
    mut signing_params_callback: SRCB,
) -> Result<Request<Vec<u8>>, SignHttpRequestError>
where
    SSCB: FnMut(SigningSettings) -> SigningSettings + Send,
    SRCB: FnMut(SigningParams) -> SigningParams + Send,
{
    let mut signing_settings = SigningSettings::default();
    signing_settings.expires_in = Some(Duration::from_secs(60 * 10));
    let signing_settings = signing_settings_callback(signing_settings);

    let signing_params = SigningParams::builder()
        .access_key(access_key_id)
        .secret_key(secret_access_key)
        .region(region)
        .service_name(service_name)
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()
        .map_err(SignHttpRequestError::MakeSigningParamsFailed)?;
    let signing_params = signing_params_callback(signing_params);

    let signable_request = SignableRequest::from(&request);
    let (signing_instructions, _signature) = sign(signable_request, &signing_params)
        .map_err(SignHttpRequestError::SignFailed)?
        .into_parts();

    signing_instructions.apply_to_request(&mut request);

    Ok(request)
}

//
#[derive(Debug)]
pub enum SignHttpRequestError {
    MakeSigningParamsFailed(AwsSigv4SigningParamsBuildError),
    SignFailed(AwsSigv4HttpRequestSignError),
}
impl fmt::Display for SignHttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl error::Error for SignHttpRequestError {}
