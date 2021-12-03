// https://docs.aws.amazon.com/rekognition/latest/dg/http-headers.html
pub const REQUIRED_HEADER_CONTENT_TYPE_VALUE: &str = "application/x-amz-json-1.1";

// https://docs.aws.amazon.com/rekognition/latest/dg/http-headers.html
pub const REQUIRED_HEADER_X_AMZ_TARGET_KEY: &str = "X-Amz-Target";

// https://docs.aws.amazon.com/rekognition/latest/dg/http-headers.html
pub fn required_header_x_amz_target_value(operation: &str) -> String {
    format!("RekognitionService.{}", operation)
}
