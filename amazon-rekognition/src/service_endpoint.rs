//! [Amazon Rekognition endpoints and quotas](https://docs.aws.amazon.com/general/latest/gr/rekognition.html)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceEndpoint<'a> {
    //
    USEastOhio,
    USEastOhioFIPS,
    USEastNVirginia,
    USEastNVirginiaFIPS,
    USWestNCalifornia,
    USWestNCaliforniaFIPS,
    USWestOregon,
    USWestOregonFIPS,
    //
    AsiaPacificMumbai,
    //
    // TODO
    //
    Other { region: &'a str, url: &'a str },
}

impl<'a> ServiceEndpoint<'a> {
    pub fn other(region: &'a str, url: &'a str) -> Self {
        Self::Other { region, url }
    }

    pub fn region(&self) -> &str {
        match self {
            Self::USEastOhio | Self::USEastOhioFIPS => "us-east-2",
            Self::USEastNVirginia | Self::USEastNVirginiaFIPS => "us-east-1",
            Self::USWestNCalifornia | Self::USWestNCaliforniaFIPS => "us-west-1",
            Self::USWestOregon | Self::USWestOregonFIPS => "us-west-2",
            Self::AsiaPacificMumbai => "ap-south-1",
            Self::Other { region, url: _ } => region,
        }
    }

    pub fn url(&self) -> &str {
        match self {
            Self::USEastOhio => "https://rekognition.us-east-2.amazonaws.com",
            Self::USEastOhioFIPS => "https://rekognition-fips.us-east-2.amazonaws.com",
            Self::USEastNVirginia => "https://rekognition.us-east-1.amazonaws.com",
            Self::USEastNVirginiaFIPS => "https://rekognition-fips.us-east-1.amazonaws.com",
            Self::USWestNCalifornia => "https://rekognition.us-west-1.amazonaws.com",
            Self::USWestNCaliforniaFIPS => "https://rekognition-fips.us-west-1.amazonaws.com",
            Self::USWestOregon => "https://rekognition.us-west-2.amazonaws.com",
            Self::USWestOregonFIPS => "https://rekognition-fips.us-west-2.amazonaws.com",
            Self::AsiaPacificMumbai => "https://rekognition.ap-south-1.amazonaws.com",
            Self::Other { region: _, url } => url,
        }
    }
}
