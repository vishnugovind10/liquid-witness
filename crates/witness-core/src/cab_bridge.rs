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
    pub descriptor_scope: String,
}

impl CabBundle {
    pub fn from_diff(
        claim: &IssuerClaim,
        observed: ObservedState,
        diff: DiffResult,
        network: impl Into<String>,
        descriptor_scope: impl Into<String>,
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
            reasons: diff.reasons,
            evidence: EvidenceSource {
                mode: mode.to_string(),
                source: "liquid-witness".to_string(),
                descriptor_scope: descriptor_scope.into(),
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
        };
        let diff = diff_claim(&claim, &observed).unwrap();
        let bundle =
            CabBundle::from_diff(&claim, observed, diff, "testnet", "watch-only descriptor");

        assert_eq!(bundle.verdict, Verdict::Demo);
        assert_eq!(bundle.claim.claim_sha256, claim_hash(&claim));
        assert_eq!(bundle.evidence.mode, "DEMO");
    }
}
