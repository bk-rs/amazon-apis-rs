//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Image.html

use super::s3_object::S3Object;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Image {
    #[serde(rename = "Bytes")]
    Bytes(Vec<u8>),
    #[serde(rename = "S3Object")]
    S3Object(S3Object),
}
