use crate::verdict::Verdict;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HolderAmount {
    pub category: String,
    pub amount: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssuerClaim {
    pub asset_id: String,
    pub total_supply: u64,
    pub holders: Vec<HolderAmount>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservedState {
    pub asset_id: String,
    pub total_supply: u64,
    pub holders: Vec<HolderAmount>,
    pub complete: bool,
    pub demo: bool,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_evidence: Option<LiveEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LiveEvidence {
    pub endpoint: String,
    pub descriptor_scope: String,
    pub tx_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gaid_redacted: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiffResult {
    pub verdict: Verdict,
    pub reasons: Vec<String>,
    pub observed_total_supply: u64,
    pub claimed_total_supply: u64,
}

#[derive(Debug, Error)]
pub enum RecomputeError {
    #[error("claim asset id {claim} does not match observed asset id {observed}")]
    AssetIdMismatch { claim: String, observed: String },
}

pub fn diff_claim(
    claim: &IssuerClaim,
    observed: &ObservedState,
) -> Result<DiffResult, RecomputeError> {
    if claim.asset_id != observed.asset_id {
        return Err(RecomputeError::AssetIdMismatch {
            claim: claim.asset_id.clone(),
            observed: observed.asset_id.clone(),
        });
    }

    if observed.demo {
        return Ok(DiffResult {
            verdict: Verdict::Demo,
            reasons: vec![
                "fixture-backed run; no live Liquid/LWK round trip was performed".to_string(),
            ],
            observed_total_supply: observed.total_supply,
            claimed_total_supply: claim.total_supply,
        });
    }

    if !observed.complete {
        return Ok(DiffResult {
            verdict: Verdict::Incomplete,
            reasons: vec!["scan did not cover the full issuer claim".to_string()],
            observed_total_supply: observed.total_supply,
            claimed_total_supply: claim.total_supply,
        });
    }

    let mut reasons = Vec::new();
    if claim.total_supply != observed.total_supply {
        reasons.push(format!(
            "total supply mismatch: claimed {}, observed {}",
            claim.total_supply, observed.total_supply
        ));
    }

    let claimed_holders = normalize_holders(&claim.holders);
    let observed_holders = normalize_holders(&observed.holders);
    for category in claimed_holders.keys().chain(observed_holders.keys()) {
        let claimed = claimed_holders.get(category).copied().unwrap_or_default();
        let observed = observed_holders.get(category).copied().unwrap_or_default();
        if claimed != observed {
            reasons.push(format!(
                "holder category {category} mismatch: claimed {claimed}, observed {observed}"
            ));
        }
    }

    let verdict = if reasons.is_empty() {
        Verdict::Verified
    } else {
        Verdict::Mismatch
    };

    Ok(DiffResult {
        verdict,
        reasons,
        observed_total_supply: observed.total_supply,
        claimed_total_supply: claim.total_supply,
    })
}

fn normalize_holders(holders: &[HolderAmount]) -> BTreeMap<&str, u64> {
    let mut normalized = BTreeMap::new();
    for holder in holders {
        *normalized.entry(holder.category.as_str()).or_insert(0) += holder.amount;
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::{diff_claim, HolderAmount, IssuerClaim, ObservedState};
    use crate::Verdict;

    fn claim() -> IssuerClaim {
        IssuerClaim {
            asset_id: "01".repeat(32),
            total_supply: 100,
            holders: vec![
                HolderAmount {
                    category: "qualified".to_string(),
                    amount: 80,
                },
                HolderAmount {
                    category: "treasury".to_string(),
                    amount: 20,
                },
            ],
        }
    }

    fn observed(complete: bool, demo: bool) -> ObservedState {
        ObservedState {
            asset_id: "01".repeat(32),
            total_supply: 100,
            holders: vec![
                HolderAmount {
                    category: "treasury".to_string(),
                    amount: 20,
                },
                HolderAmount {
                    category: "qualified".to_string(),
                    amount: 80,
                },
            ],
            complete,
            demo,
            source: "fixture".to_string(),
            live_evidence: None,
        }
    }

    #[test]
    fn verified_when_complete_observation_matches_claim() {
        let result = diff_claim(&claim(), &observed(true, false)).unwrap();
        assert_eq!(result.verdict, Verdict::Verified);
        assert!(result.reasons.is_empty());
    }

    #[test]
    fn mismatch_when_supply_or_distribution_differs() {
        let mut observed = observed(true, false);
        observed.total_supply = 99;
        observed.holders[0].amount = 19;
        let result = diff_claim(&claim(), &observed).unwrap();
        assert_eq!(result.verdict, Verdict::Mismatch);
        assert_eq!(result.reasons.len(), 3);
    }

    #[test]
    fn incomplete_takes_precedence_over_mismatch() {
        let mut observed = observed(false, false);
        observed.total_supply = 99;
        let result = diff_claim(&claim(), &observed).unwrap();
        assert_eq!(result.verdict, Verdict::Incomplete);
    }

    #[test]
    fn demo_is_not_promoted_to_verified() {
        let result = diff_claim(&claim(), &observed(true, true)).unwrap();
        assert_eq!(result.verdict, Verdict::Demo);
    }
}
