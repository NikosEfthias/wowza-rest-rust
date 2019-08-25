use serde::{
    Deserialize,
    Serialize,
};
#[allow(bad_style)]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct StreamRecorderConfig
{
    pub timeScale: String,
    pub instanceName: String,
    pub fileVersionDelegateName: String,
    pub serverName: String,
    pub recorderName: String,
    pub currentSize: isize,
    pub segmentSchedule: String,
    pub startOnKeyFrame: bool,
    pub outputPath: String,
    pub currentFile: String,
    pub saveFieldList: Vec<String>,
    pub defaultAudioSearchPosition: bool,
    pub recordData: bool,
    pub applicationName: String,
    pub moveFirstVideoFrameToZero: bool,
    pub recorderErrorString: String,
    pub segmentSize: isize,
    pub defaultRecorder: bool,
    pub splitOnTcDiscontinuity: bool,
    pub version: String,
    pub skipKeyFrameUntilAudioTimeout: isize,
    pub baseFile: String,
    pub segmentDuration: isize,
    pub recordingStartTime: String,
    pub fileTemplate: String,
    pub backBufferTime: isize,
    pub segmentationType: String,
    pub currentDuration: isize,
    pub fileFormat: String,
    pub recorderState: String,
    pub option: String,
    #[serde(skip)]
    pub vhostName: String,
}
pub(crate) fn default_headers() -> Vec<(&'static str, &'static str)>
{
    vec![
        ("Content-Type", "application/json"),
        ("Accept", "application/json"),
    ]
}
pub enum ResponseStatus
{
    Ok,
    BadRequest,
    PaymentRequired,
    NotFound,
    UnsupportedMediaType,
    Created,
    Conflict,
}
