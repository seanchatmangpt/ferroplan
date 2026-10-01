pub fn exponential(base: u64, attempt: u32, cap: u64) -> u64 {
    base.saturating_mul(1u64.checked_shl(attempt.min(62)).unwrap_or(u64::MAX))
        .min(cap)
}
