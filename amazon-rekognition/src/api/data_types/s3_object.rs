//! https://docs.aws.amazon.com/rekognition/latest/dg/API_S3Object.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct S3Object {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: Option<String>,
}
