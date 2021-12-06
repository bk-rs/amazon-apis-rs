//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Image.html

use std::{cmp::min, fmt};

use serde::{Deserialize, Serialize};

use super::s3_object::S3Object;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}
impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(bytes) = &self.bytes {
            f.debug_struct("Image")
                .field(
                    "bytes",
                    &format_args!("{}...", &bytes[0..min(10, bytes.len())]),
                )
                .field("s3_object", &self.s3_object)
                .finish()
        } else {
            f.debug_struct("Image")
                .field("bytes", &self.bytes)
                .field("s3_object", &self.s3_object)
                .finish()
        }
    }
}

impl Image {
    pub fn with_bytes(binary_data: Vec<u8>) -> Self {
        Self {
            bytes: Some(base64::encode(binary_data)),
            s3_object: None,
        }
    }
    pub fn with_s3_object(s3_object: S3Object) -> Self {
        Self {
            bytes: None,
            s3_object: Some(s3_object),
        }
    }
}
