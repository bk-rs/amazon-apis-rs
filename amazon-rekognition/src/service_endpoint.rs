//! [Amazon Rekognition endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/rekognition.html)

use std::borrow::{Borrow, Cow};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceEndpoint {
    //
    USEastOhio,
    USEastOhioFIPS,
    //
    // TODO
    //
    //
    AsiaPacificMumbai,
    //
    // TODO
    //
    Other {
        region: Cow<'static, str>,
        url: Cow<'static, str>,
    },
}

impl ServiceEndpoint {
    pub fn region(&self) -> &str {
        match self {
            Self::USEastOhio | Self::USEastOhioFIPS => "us-east-2",
            Self::AsiaPacificMumbai => "ap-south-1",
            Self::Other { region, url: _ } => region.borrow(),
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::USEastOhio => "https://rekognition.us-east-2.amazonaws.com",
            Self::USEastOhioFIPS => "https://rekognition-fips.us-east-2.amazonaws.com",
            Self::AsiaPacificMumbai => "https://rekognition.ap-south-1.amazonaws.com",
            Self::Other { region: _, url } => url,
        }
    }
}
