//! A clock that doesn't get you killed in the browser. `std::time::Instant::now()`
//! panics outright on `wasm32-unknown-unknown` — no time backend there — and
//! every read this engine takes off the clock is measurement, reporting,
//! never a load-bearing behavior. So on wasm the clock just freezes at
//! zero instead of blowing up, which is exactly the number a browser tab
//! should report anyway. Found the hard way: the in-page demo died at
//! `search_from`'s phase-attribution timer the moment any solve reached
//! the best-first fallback.

/// A monotonic timestamp. Reads real time everywhere except wasm, where it
/// goes still and reports zero.
#[derive(Clone, Copy)]
pub struct Clock {
    #[cfg(not(target_arch = "wasm32"))]
    t0: std::time::Instant,
}

impl Clock {
    #[inline]
    pub fn now() -> Self {
        Clock {
            #[cfg(not(target_arch = "wasm32"))]
            t0: std::time::Instant::now(),
        }
    }

    #[inline]
    pub fn elapsed_ms(&self) -> u128 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.t0.elapsed().as_millis()
        }
        #[cfg(target_arch = "wasm32")]
        {
            0
        }
    }

    #[inline]
    pub fn elapsed_us(&self) -> u128 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.t0.elapsed().as_micros()
        }
        #[cfg(target_arch = "wasm32")]
        {
            0
        }
    }

    #[inline]
    pub fn elapsed_secs(&self) -> f64 {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.t0.elapsed().as_secs_f64()
        }
        #[cfg(target_arch = "wasm32")]
        {
            0.0
        }
    }
}
