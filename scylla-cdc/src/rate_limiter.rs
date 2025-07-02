use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;

static RATE_LIMIT: u32 = 10000;
// Global rate limiter:
pub static GLOBAL_RATE_LIMITER: once_cell::sync::Lazy<Arc<DefaultDirectRateLimiter>> =
    once_cell::sync::Lazy::new(|| {
        let quota = Quota::per_second(NonZeroU32::new(RATE_LIMIT).unwrap());
        Arc::new(RateLimiter::direct(quota))
    });
