//! https://docs.aws.amazon.com/rekognition/latest/dg/API_S3Object.html

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct S3Object {
    pub bucket: String,
    pub name: String,
    pub version: Option<String>,
}
