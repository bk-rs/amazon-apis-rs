//! [API Reference](https://docs.aws.amazon.com/rekognition/latest/dg/API_Reference.html)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionCategory {
    Image,
    CustomLabels,
    VideoStoredVideo,
    VideoStreamingVideo,
}
