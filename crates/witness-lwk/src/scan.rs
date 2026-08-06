use crate::descriptor::{DescriptorError, WatchOnlyDescriptor};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use witness_core::ObservedState;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScanRequest {
    pub asset_id: String,
    pub descriptor: String,
    pub network: String,
    pub electrum_url: Option<String>,
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error(transparent)]
    Descriptor(#[from] DescriptorError),
    #[error("fixture read failed: {0}")]
    FixtureRead(String),
    #[error("fixture parse failed: {0}")]
    FixtureParse(String),
}

pub fn scan_fixture(path: &Path) -> Result<ObservedState, ScanError> {
    let contents =
        fs::read_to_string(path).map_err(|error| ScanError::FixtureRead(error.to_string()))?;
    serde_json::from_str(&contents).map_err(|error| ScanError::FixtureParse(error.to_string()))
}

pub fn scan_live_incomplete(request: &ScanRequest) -> Result<ObservedState, ScanError> {
    let descriptor = WatchOnlyDescriptor::parse(request.descriptor.clone())?;
    Ok(ObservedState {
        asset_id: request.asset_id.clone(),
        total_supply: 0,
        holders: Vec::new(),
        complete: false,
        demo: false,
        source: format!(
            "live LWK scan boundary reserved for {}; descriptor {}",
            request
                .electrum_url
                .as_deref()
                .unwrap_or("elements-testnet.blockstream.info:50002"),
            descriptor.redacted_scope()
        ),
    })
}

#[cfg(feature = "live-lwk")]
pub mod live_lwk_symbols {
    pub fn lwk_feature_is_linked() -> &'static str {
        let _ = core::any::type_name::<lwk_wollet::ElectrumUrl>();
        "lwk_wollet"
    }
}

#[cfg(test)]
mod tests {
    use super::{scan_fixture, scan_live_incomplete, ScanRequest};
    use std::path::PathBuf;

    #[test]
    fn fixture_scan_loads_observed_state() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/demo-observed-state.json");
        let state = scan_fixture(&path).unwrap();
        assert!(state.demo);
        assert!(state.complete);
    }

    #[test]
    fn live_placeholder_is_incomplete_not_verified() {
        let state = scan_live_incomplete(&ScanRequest {
            asset_id: "ab".repeat(32),
            descriptor: "ct(elwpk([00000000/84h/1h/0h]tpub.../0/*))".to_string(),
            network: "testnet".to_string(),
            electrum_url: None,
        })
        .unwrap();
        assert!(!state.complete);
        assert!(!state.demo);
    }
}
