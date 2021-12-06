/*
RUST_BACKTRACE=1 RUST_LOG=trace cargo run -p amazon-rekognition-demo --bin amazon_rekognition_detect_labels -- 'YOUR_ACCESS_KEY_ID' 'YOUR_SECRET_ACCESS_KEY'
*/

use std::{env, error};

use amazon_rekognition::{
    api::{
        actions::{detect_labels, Action, DetectLabelsRequestBody},
        data_types::Image,
    },
    ServiceEndpoint,
};
use futures_lite::future::block_on;
use http_api_isahc_client::{Client as _, IsahcClient};
use serde_json::{json, Map, Value};

fn main() -> Result<(), Box<dyn error::Error>> {
    pretty_env_logger::init();

    block_on(run())
}

async fn run() -> Result<(), Box<dyn error::Error>> {
    let access_key_id = env::args().nth(1).unwrap();
    let secret_access_key = env::args().nth(2).unwrap();
    let image_bytes = include_bytes!("../../tests/image_files/1.jpeg");

    let client = IsahcClient::new()?;

    //
    let detect_labels = detect_labels::new(
        &access_key_id,
        &secret_access_key,
        &ServiceEndpoint::USEastOhio,
        DetectLabelsRequestBody {
            image: Image::with_bytes(image_bytes.to_vec()),
            max_labels: None,
            min_confidence: None,
        },
    );

    let ret = client.respond_endpoint(&detect_labels).await?;
    println!("{:?}", ret);

    //
    let action: Action<_, Map<String, Value>> = Action::new(
        &access_key_id,
        &secret_access_key,
        &ServiceEndpoint::USEastOhio,
        json!({
            "Image": {
                "Bytes": base64::encode(image_bytes.to_vec()),
            }
        }),
        "DetectLabels",
    );
    let ret = client.respond_endpoint(&action).await?;
    println!("{:?}", ret);

    Ok(())
}
