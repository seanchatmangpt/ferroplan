# v26.8.1 full-planning reference domains

## Repository uncertainty

`repository_uncertainty/` separates controlled production from stochastic
inspection. A candidate is inspected, a defect may be observed and repaired,
and publication is admitted only after validation. The planner selects a policy;
it does not publish.

## TCPS application pipeline

`tcps_application_pipeline/` models the recruiter and employer as uncertain
downstream processes. Manufacturing the fit package is deterministic. Recruiter
submission, client review, and production assessment are stochastic. The
`incompatible` fact is an admitted unsafe-state label for risk-constrained
verification.

Both examples are deliberately small oracle domains. They are fixtures for
policy synthesis, simulation, verification, session observation, and receipts;
they are not empirical probability claims about repositories or hiring.
