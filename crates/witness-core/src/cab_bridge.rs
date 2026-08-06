use crate::recompute::{DiffResult, IssuerClaim, ObservedState};
use crate::verdict::Verdict;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CabBundle {
    pub cab_version: String,
    pub subject: CabSubject,
    pub claim: CabClaim,
    pub observed: ObservedState,
    pub verdict: Verdict,
    pub supply_delta: i64,
    pub reasons: Vec<String>,
    pub evidence: EvidenceSource,
    pub generated_at: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CabSubject {
    pub asset_id: String,
    pub network: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CabClaim {
    pub total_supply: u64,
    pub claim_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceSource {
    pub mode: String,
    pub source: String,
}

impl CabBundle {
    pub fn from_diff(
        claim: &IssuerClaim,
        observed: ObservedState,
        diff: DiffResult,
        network: impl Into<String>,
        _descriptor_scope: impl Into<String>,
    ) -> Self {
        let generated_at = OffsetDateTime::now_utc()
            .format(&Rfc3339)
            .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
        let mode = if observed.demo {
            "DEMO"
        } else {
            "LIVE_OR_REPLAY"
        };
        CabBundle {
            cab_version: "0.1-compatible".to_string(),
            subject: CabSubject {
                asset_id: claim.asset_id.clone(),
                network: network.into(),
            },
            claim: CabClaim {
                total_supply: claim.total_supply,
                claim_sha256: claim_hash(claim),
            },
            observed,
            verdict: diff.verdict,
            supply_delta: diff.observed_total_supply as i64 - diff.claimed_total_supply as i64,
            reasons: diff.reasons,
            evidence: EvidenceSource {
                mode: mode.to_string(),
                source: "liquid-witness".to_string(),
            },
            generated_at,
        }
    }
}

pub fn claim_hash(claim: &IssuerClaim) -> String {
    let bytes = serde_json::to_vec(claim).expect("issuer claim serialization is infallible");
    hex::encode(Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::{claim_hash, CabBundle};
    use crate::{diff_claim, HolderAmount, IssuerClaim, ObservedState, Verdict};

    #[test]
    fn cab_bundle_preserves_demo_boundary() {
        let claim = IssuerClaim {
            asset_id: "ab".repeat(32),
            total_supply: 5,
            holders: vec![HolderAmount {
                category: "issuer".to_string(),
                amount: 5,
            }],
        };
        let observed = ObservedState {
            asset_id: claim.asset_id.clone(),
            total_supply: 5,
            holders: claim.holders.clone(),
            complete: true,
            demo: true,
            source: "tests/fixtures/demo-observed-state.json".to_string(),
            live_evidence: None,
        };
        let diff = diff_claim(&claim, &observed).unwrap();
        let bundle =
            CabBundle::from_diff(&claim, observed, diff, "testnet", "watch-only descriptor");

        assert_eq!(bundle.verdict, Verdict::Demo);
        assert_eq!(bundle.claim.claim_sha256, claim_hash(&claim));
        assert_eq!(bundle.evidence.mode, "DEMO");
        assert_eq!(bundle.supply_delta, 0);
    }

    #[test]
    fn cab_json_never_serializes_descriptor_material() {
        let secret_descriptor =
            "ct(slip77(super-secret-test-descriptor),elwpkh([abcd1234]tpub-secret/0/*))";
        let claim = IssuerClaim {
            asset_id: "cd".repeat(32),
            total_supply: 7,
            holders: vec![HolderAmount {
                category: "descriptor-scope".to_string(),
                amount: 7,
            }],
        };
        let observed = ObservedState {
            asset_id: claim.asset_id.clone(),
            total_supply: 7,
            holders: claim.holders.clone(),
            complete: true,
            demo: false,
            source: "lwk_wollet full_scan_with_electrum_client".to_string(),
            live_evidence: Some(crate::LiveEvidence {
                endpoint: "elements-testnet.blockstream.info:50002".to_string(),
                tx_count: 1,
                txid: Some("11".repeat(32)),
                gaid_redacted: Some("gaid...redacted".to_string()),
            }),
        };
        let diff = diff_claim(&claim, &observed).unwrap();
        let bundle = CabBundle::from_diff(&claim, observed, diff, "testnet", secret_descriptor);
        let json = serde_json::to_string(&bundle).unwrap();

        assert!(!json.contains(secret_descriptor));
        assert!(!json.contains("super-secret-test-descriptor"));
        assert!(!json.contains("tpub-secret"));
    }
}
