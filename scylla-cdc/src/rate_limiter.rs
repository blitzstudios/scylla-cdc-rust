use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;
use std::sync::OnceLock;

static GLOBAL_RATE_LIMITER: OnceLock<Arc<DefaultDirectRateLimiter>> = OnceLock::new();

/// Inject the rate limiter from the application
pub fn set_rate_limiter(rate_limiter: Arc<DefaultDirectRateLimiter>) -> Result<(), &'static str> {
    GLOBAL_RATE_LIMITER
        .set(rate_limiter)
        .map_err(|_| "Rate limiter already set")
}

pub fn get_rate_limiter() -> &'static Arc<DefaultDirectRateLimiter> {
    GLOBAL_RATE_LIMITER
        .get()
        .expect("Rate limiter not injected. Call inject_rate_limiter() first.")
}

pub fn create_rate_limiter(rate_limit: u32) -> Arc<DefaultDirectRateLimiter> {
    let quota = Quota::per_second(NonZeroU32::new(rate_limit).unwrap());
    Arc::new(RateLimiter::direct(quota))
}
