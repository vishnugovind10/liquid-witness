use std::fs;
use std::path::PathBuf;
use witness_core::{CabBundle, Verdict};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn committed_demo_cab_replays_stable_verdict() {
    let path = repo_root().join("examples/testnet-amp-scan/output.cab");
    let bundle: CabBundle =
        serde_json::from_str(&fs::read_to_string(path).expect("demo CAB must be readable"))
            .expect("demo CAB must parse");

    assert_eq!(bundle.verdict, Verdict::Demo);
    assert_eq!(bundle.evidence.mode, "DEMO");
}

#[test]
fn committed_live_cab_replays_stable_verdict_when_present() {
    let path = repo_root().join("examples/testnet-amp-scan/live-output.cab");
    if !path.exists() {
        return;
    }

    let bundle: CabBundle =
        serde_json::from_str(&fs::read_to_string(path).expect("live CAB must be readable"))
            .expect("live CAB must parse");

    assert!(!matches!(bundle.verdict, Verdict::Demo));
    assert_eq!(bundle.evidence.mode, "LIVE_OR_REPLAY");
    assert!(bundle.observed.live_evidence.is_some());
}
