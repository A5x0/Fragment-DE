#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct IpcEnvelope {
    pub version: u32,
    pub r#type: String,
    #[serde(default)]
    pub id: Option<u64>,
    pub payload: serde_json::Value,
}
