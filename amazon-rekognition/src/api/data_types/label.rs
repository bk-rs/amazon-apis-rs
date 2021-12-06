//! https://docs.aws.amazon.com/rekognition/latest/dg/API_Label.html

use serde::{Deserialize, Serialize};

use super::{Instance, Parent};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Label {
    pub confidence: Option<f64>,
    pub instances: Option<Vec<Instance>>,
    pub name: Option<String>,
    pub parents: Option<Vec<Parent>>,
}
