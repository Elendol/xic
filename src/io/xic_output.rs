use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct XicOutput {
    #[serde(rename = "OutputMeta")]
    pub(crate) meta: Meta,
    #[serde(rename = "Content")]
    pub(crate) content: Vec<OutputContent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Meta {
    base64: bool,
    timeunit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub(crate) enum OutputContent {
    Content {
        #[serde(rename = "Meta")]
        meta: QueryMeta,
        #[serde(rename = "RetentionTimes")]
        retention_times: Option<Vec<f32>>,
        #[serde(rename = "Intensities")]
        intensities: Option<Vec<f32>>,
    },
    ContentBase64 {
        #[serde(rename = "Meta")]
        meta: QueryMeta,
        #[serde(rename = "RetentionTimes")]
        retention_times: Option<String>,
        #[serde(rename = "Intensities")]
        intensities: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct QueryMeta {
    #[serde(flatten)]
    pub(crate) mz: MzRange,
    #[serde(flatten)]
    pub(crate) rt: RtRange,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct MzRange {
    #[serde(rename = "MzStart")]
    pub(crate) start: f32,
    #[serde(rename = "MzEnd")]
    pub(crate) end: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct RtRange {
    #[serde(rename = "RtStart")]
    pub(crate) start: f32,
    #[serde(rename = "RtEnd")]
    pub(crate) end: f32,
}
