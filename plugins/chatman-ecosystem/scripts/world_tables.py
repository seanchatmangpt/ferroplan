"""GENERATED from ontology/ferroplan-self-host.ttl by ggen rule self-host-tables. Do not edit."""

PREDICATES = {
    "epistemic": {
        "latent": "epistemic-latent",
        "observed": "epistemic-observed",
        "admitted": "epistemic-admitted",
    },
    "allocation": {
        "unallocated": "unallocated",
        "allocated": "allocated",
    },
    "planning": {
        "unplanned": "unplanned",
        "candidate": "candidate-plan",
        "validated": "validated-plan",
    },
    "actuation": {
        "sealed": "actuation-sealed",
        "manufacturing": "manufacturing",
        "receipted": "receipted",
        "publishable": "publishable",
    },
    "drift": {
        "stable": "stable",
        "drifted": "drifted",
        "refused": "refused",
    },
    "conformance": {
        "unknown": "config-unknown",
        "nonconformant": "config-nonconformant",
        "conformant": "config-conformant",
    },
}

GOALS = {
    "plan": [
        "candidate-plan",
    ],
    "validate": [
        "validated-plan",
        "validator-green",
    ],
    "receipt": [
        "receipt-bound",
        "validator-green",
    ],
    "publish": [
        "draft-pr-open",
    ],
}
