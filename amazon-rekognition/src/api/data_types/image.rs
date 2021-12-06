//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Image.html

use serde::{Deserialize, Serialize};

use super::s3_object::S3Object;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    pub bytes: Option<String>,
    pub s3_object: Option<S3Object>,
    #[serde(skip)]
    _priv: (),
}
impl Image {
    pub fn with_bytes(binary_data: Vec<u8>) -> Self {
        Self {
            bytes: Some(base64::encode(binary_data)),
            s3_object: None,
            _priv: (),
        }
    }

    pub fn with_s3_object(s3_object: S3Object) -> Self {
        Self {
            bytes: None,
            s3_object: Some(s3_object),
            _priv: (),
        }
    }
}
