use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ProbabilisticOptions, ProbabilisticSolution};

use super::{PolicyVerificationReport, RiskConstraint};

const RECEIPT_SCHEMA: &str = "urn:chatman:ferroplan-policy-receipt:v1";
const RECEIPT_DOMAIN: &[u8] = b"urn:chatman:ferroplan-policy-receipt:v1\0";

fn canonicalize(value: Value) -> Value {
    match value {
        Value::Array(values) => Value::Array(values.into_iter().map(canonicalize).collect()),
        Value::Object(values) => {
            let mut entries = values.into_iter().collect::<Vec<_>>();
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(key, value)| (key, canonicalize(value)))
                    .collect(),
            )
        }
        other => other,
    }
}

pub fn canonical_digest<T: Serialize>(value: &T) -> String {
    let value = serde_json::to_value(value).unwrap_or(Value::Null);
    let bytes = serde_json::to_vec(&canonicalize(value)).unwrap_or_default();
    let mut hasher = blake3::Hasher::new();
    hasher.update(RECEIPT_DOMAIN);
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(&bytes);
    hasher.finalize().to_hex().to_string()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PolicyReceipt {
    pub schema: String,
    pub model_digest: String,
    pub policy_digest: String,
    pub verifier_digest: String,
    pub objective: crate::ProbabilisticObjective,
    pub constraints: Vec<RiskConstraint>,
    pub predecessor: Option<String>,
    pub receipt_digest: String,
}

#[derive(Serialize)]
struct ReceiptPayload<'a> {
    schema: &'a str,
    model_digest: &'a str,
    policy_digest: &'a str,
    verifier_digest: &'a str,
    objective: crate::ProbabilisticObjective,
    constraints: &'a [RiskConstraint],
    predecessor: &'a Option<String>,
}

pub fn bind_policy_receipt(
    domain: &str,
    problem: &str,
    options: &ProbabilisticOptions,
    constraints: &[RiskConstraint],
    solution: &ProbabilisticSolution,
    verifier: &PolicyVerificationReport,
    predecessor: Option<String>,
) -> PolicyReceipt {
    let model_digest = canonical_digest(&(domain, problem, options, constraints));
    let policy_digest = canonical_digest(solution);
    let verifier_digest = canonical_digest(verifier);
    let receipt_digest = canonical_digest(&ReceiptPayload {
        schema: RECEIPT_SCHEMA,
        model_digest: &model_digest,
        policy_digest: &policy_digest,
        verifier_digest: &verifier_digest,
        objective: solution.objective,
        constraints,
        predecessor: &predecessor,
    });
    PolicyReceipt {
        schema: RECEIPT_SCHEMA.into(),
        model_digest,
        policy_digest,
        verifier_digest,
        objective: solution.objective,
        constraints: constraints.to_vec(),
        predecessor,
        receipt_digest,
    }
}

pub fn verify_policy_receipt(receipt: &PolicyReceipt) -> bool {
    if receipt.schema != RECEIPT_SCHEMA {
        return false;
    }
    canonical_digest(&ReceiptPayload {
        schema: RECEIPT_SCHEMA,
        model_digest: &receipt.model_digest,
        policy_digest: &receipt.policy_digest,
        verifier_digest: &receipt.verifier_digest,
        objective: receipt.objective,
        constraints: &receipt.constraints,
        predecessor: &receipt.predecessor,
    }) == receipt.receipt_digest
}

pub fn verify_policy_chain(receipts: &[PolicyReceipt]) -> bool {
    receipts.iter().enumerate().all(|(index, receipt)| {
        verify_policy_receipt(receipt)
            && match index {
                0 => receipt.predecessor.is_none(),
                _ => {
                    receipt.predecessor.as_deref()
                        == receipts
                            .get(index - 1)
                            .map(|previous| previous.receipt_digest.as_str())
                }
            }
    })
}
