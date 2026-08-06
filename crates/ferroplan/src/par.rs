//! Bare-metal wetware: data-parallel primitives riding raw `std::thread::scope`,
//! no external deps, no chrome. Worker count reads off `FFDP_THREADS` if the
//! console's set it, else counts cores itself; drop to `threads <= 1` and the
//! rig folds back to a straight sequential pass — same output, no matter how
//! many hands are on deck.
//!
//! Two chokes keep the parallel run from eating its own gains:
//!  - `MIN_PAR`: a frontier thinner than this runs SERIAL, full stop. Small
//!    jobs and the dying tail of big ones never foot the thread-spawn tax.
//!    Output's identical either way — the choke just decides who pays for it.
//!  - `MAX_DEFAULT_THREADS`: auto-picked worker count gets capped — the scaling
//!    curve flatlines past ~4 cores (Amdahl's ghost: serial successor-gen,
//!    dedup, heap-work bottlenecking the rest). Past that, more threads is
//!    just spinning rotors for nobody. Hand it an explicit `FFDP_THREADS` and
//!    the cap's off — you take the wheel, you take the risk.

/// Under this headcount, run it solo — spawning threads is dead weight.
pub const MIN_PAR: usize = 32;
/// Ceiling on auto-picked workers. Curve goes flat past ~4; padding for margin.
const MAX_DEFAULT_THREADS: usize = 6;

/// Reads the worker count off the wire: `FFDP_THREADS` override (no ceiling),
/// otherwise `min(cores, MAX_DEFAULT_THREADS)`.
pub fn num_threads() -> usize {
    if let Ok(s) = std::env::var("FFDP_THREADS") {
        if let Ok(n) = s.parse::<usize>() {
            if n >= 1 {
                return n;
            }
        }
    }
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(MAX_DEFAULT_THREADS)
}

/// Fans `f` out across `threads` scoped hands, one contiguous chunk per worker,
/// order held the whole way. Results stitch back seamless — output's a dead
/// ringer for sequential, only the clock runs parallel.
pub fn par_map<T, R, F>(items: &[T], threads: usize, f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    let n = items.len();
    if threads <= 1 || n < MIN_PAR {
        return items.iter().map(&f).collect();
    }
    let t = threads.min(n);
    let chunk = n.div_ceil(t);
    let chunks: Vec<&[T]> = items.chunks(chunk).collect();
    let f = &f;
    let mut parts: Vec<Vec<R>> = Vec::with_capacity(chunks.len());
    std::thread::scope(|scope| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|c| scope.spawn(move || c.iter().map(f).collect::<Vec<R>>()))
            .collect();
        for h in handles {
            parts.push(h.join().expect("worker thread panicked"));
        }
    });
    parts.into_iter().flatten().collect()
}

/// `par_map`'s cousin, sharper teeth: each worker forges its own private
/// `state` off `init` (scratch buffers, whatever needs reusing) and runs it
/// through `f` for the whole chunk — allocation cost spread thin across the
/// run instead of paid per item. Input order survives intact.
pub fn par_map_with<T, R, S, I, F>(items: &[T], threads: usize, init: I, f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    I: Fn() -> S + Sync,
    F: Fn(&mut S, &T) -> R + Sync,
{
    let n = items.len();
    if threads <= 1 || n < MIN_PAR {
        let mut s = init();
        return items.iter().map(|x| f(&mut s, x)).collect();
    }
    let t = threads.min(n);
    let chunk = n.div_ceil(t);
    let chunks: Vec<&[T]> = items.chunks(chunk).collect();
    let init = &init;
    let f = &f;
    let mut parts: Vec<Vec<R>> = Vec::with_capacity(chunks.len());
    std::thread::scope(|scope| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|c| {
                scope.spawn(move || {
                    let mut s = init();
                    c.iter().map(|x| f(&mut s, x)).collect::<Vec<R>>()
                })
            })
            .collect();
        for h in handles {
            parts.push(h.join().expect("worker thread panicked"));
        }
    });
    parts.into_iter().flatten().collect()
}
