use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AmpMode {
    Amp0,
    Amp2,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AmpAssetScope {
    pub asset_id: String,
    pub mode: AmpMode,
    pub network: String,
}

impl AmpAssetScope {
    pub fn testnet_amp0(asset_id: impl Into<String>) -> Self {
        Self {
            asset_id: asset_id.into(),
            mode: AmpMode::Amp0,
            network: "testnet".to_string(),
        }
    }
}
