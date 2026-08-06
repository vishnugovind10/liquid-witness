use crate::descriptor::{DescriptorError, WatchOnlyDescriptor};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
#[cfg(feature = "live-lwk")]
use witness_core::HolderAmount;
use witness_core::{LiveEvidence, ObservedState};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScanRequest {
    pub asset_id: String,
    pub descriptor: String,
    pub network: String,
    pub electrum_url: Option<String>,
    pub txid: Option<String>,
    pub gaid_redacted: Option<String>,
}

#[derive(Debug, Error)]
pub enum ScanError {
    #[error(transparent)]
    Descriptor(#[from] DescriptorError),
    #[error("fixture read failed: {0}")]
    FixtureRead(String),
    #[error("fixture parse failed: {0}")]
    FixtureParse(String),
    #[error("live LWK scanning requires building with --features live-lwk")]
    LiveFeatureDisabled,
    #[error("unsupported network {0}; v0.2 live capture only supports testnet")]
    UnsupportedNetwork(String),
    #[error("live LWK scan failed: {0}")]
    LiveScan(String),
}

pub fn scan_fixture(path: &Path) -> Result<ObservedState, ScanError> {
    let contents =
        fs::read_to_string(path).map_err(|error| ScanError::FixtureRead(error.to_string()))?;
    serde_json::from_str(&contents).map_err(|error| ScanError::FixtureParse(error.to_string()))
}

pub fn scan_live_incomplete(request: &ScanRequest) -> Result<ObservedState, ScanError> {
    let _descriptor = WatchOnlyDescriptor::parse(request.descriptor.clone())?;
    Ok(ObservedState {
        asset_id: request.asset_id.clone(),
        total_supply: 0,
        holders: Vec::new(),
        complete: false,
        demo: false,
        source: format!(
            "live LWK scan requires --features live-lwk for {}",
            electrum_endpoint(request)
        ),
        live_evidence: Some(LiveEvidence {
            endpoint: electrum_endpoint(request),
            tx_count: 0,
            txid: request.txid.clone(),
            gaid_redacted: request.gaid_redacted.clone(),
        }),
    })
}

#[cfg(not(feature = "live-lwk"))]
pub fn scan_live(request: &ScanRequest) -> Result<ObservedState, ScanError> {
    let _ = WatchOnlyDescriptor::parse(request.descriptor.clone())?;
    Err(ScanError::LiveFeatureDisabled)
}

#[cfg(feature = "live-lwk")]
pub fn scan_live(request: &ScanRequest) -> Result<ObservedState, ScanError> {
    use lwk_wollet::elements::AssetId;
    use lwk_wollet::{
        full_scan_with_electrum_client, ElectrumClient, ElectrumUrl, ElementsNetwork,
        WolletBuilder, WolletDescriptor,
    };
    use std::str::FromStr;

    if request.network != "testnet" {
        return Err(ScanError::UnsupportedNetwork(request.network.clone()));
    }

    let descriptor = WatchOnlyDescriptor::parse(request.descriptor.clone())?;
    let wollet_descriptor: WolletDescriptor = descriptor
        .as_str()
        .parse()
        .map_err(|error| ScanError::LiveScan(format!("descriptor parse failed: {error}")))?;
    let mut wollet = WolletBuilder::new(ElementsNetwork::LiquidTestnet, wollet_descriptor)
        .build()
        .map_err(|error| ScanError::LiveScan(format!("wollet build failed: {error}")))?;

    let endpoint = electrum_endpoint(request);
    let electrum_url = ElectrumUrl::new(&endpoint, true, true)
        .map_err(|error| ScanError::LiveScan(format!("electrum url failed: {error}")))?;
    let mut electrum_client = ElectrumClient::new(&electrum_url)
        .map_err(|error| ScanError::LiveScan(format!("electrum client failed: {error}")))?;
    full_scan_with_electrum_client(&mut wollet, &mut electrum_client)
        .map_err(|error| ScanError::LiveScan(format!("full scan failed: {error}")))?;

    let asset_id = AssetId::from_str(&request.asset_id)
        .map_err(|error| ScanError::LiveScan(format!("asset id parse failed: {error}")))?;
    let balance = wollet
        .balance()
        .map_err(|error| ScanError::LiveScan(format!("balance failed: {error}")))?;
    let amount = balance.get(&asset_id).copied().unwrap_or_default();
    let tx_count = wollet
        .transactions()
        .map_err(|error| ScanError::LiveScan(format!("transactions failed: {error}")))?
        .len();

    Ok(ObservedState {
        asset_id: request.asset_id.clone(),
        total_supply: amount,
        holders: vec![HolderAmount {
            category: "descriptor-scope".to_string(),
            amount,
        }],
        complete: true,
        demo: false,
        source: "lwk_wollet full_scan_with_electrum_client".to_string(),
        live_evidence: Some(LiveEvidence {
            endpoint,
            tx_count,
            txid: request.txid.clone(),
            gaid_redacted: request.gaid_redacted.clone(),
        }),
    })
}

fn electrum_endpoint(request: &ScanRequest) -> String {
    request
        .electrum_url
        .clone()
        .unwrap_or_else(|| "elements-testnet.blockstream.info:50002".to_string())
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
            txid: None,
            gaid_redacted: None,
        })
        .unwrap();
        assert!(!state.complete);
        assert!(!state.demo);
    }
}
