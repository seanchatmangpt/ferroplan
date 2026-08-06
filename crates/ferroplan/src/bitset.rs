//! Fixed-width flags over `u64` words — the fact-layer of a state, packed
//! tight. Word-oriented so the applicability check and the apply step run as
//! bare bitwise loops, and every state stays small enough to hash and
//! dedup at speed when the search fans out across threads.

#[inline]
pub fn words_for(n_bits: usize) -> usize {
    n_bits.div_ceil(64)
}

#[inline]
pub fn test(w: &[u64], i: usize) -> bool {
    (w[i >> 6] >> (i & 63)) & 1 != 0
}

#[inline]
pub fn set(w: &mut [u64], i: usize) {
    w[i >> 6] |= 1u64 << (i & 63);
}

#[inline]
pub fn clear(w: &mut [u64], i: usize) {
    w[i >> 6] &= !(1u64 << (i & 63));
}

/// Count the flags standing across the whole word array.
pub fn count(w: &[u64]) -> usize {
    w.iter().map(|x| x.count_ones() as usize).sum()
}
