//! Shared test helpers for integration tests that call the live Open-Meteo API.

use std::future::Future;
use std::time::Duration;

/// Retries a live-API call up to 3 attempts total, with a short backoff between
/// attempts, to absorb transient network failures/timeouts rather than the
/// live-API calling test flaking outright. Does not retry on success or once
/// attempts are exhausted — the final `Result` (success or error) is returned
/// as-is so assertions on error content still work correctly.
pub async fn retry_network<F, Fut, T, E>(mut f: F) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    const MAX_ATTEMPTS: u32 = 3;
    let mut attempt = 0;
    loop {
        let result = f().await;
        attempt += 1;
        if result.is_ok() || attempt >= MAX_ATTEMPTS {
            return result;
        }
        tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
    }
}
