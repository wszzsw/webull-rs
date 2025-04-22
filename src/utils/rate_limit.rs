use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Rate limiter for API requests.
pub struct RateLimiter {
    /// Maximum number of requests per minute
    requests_per_minute: u32,

    /// Request timestamps
    timestamps: Arc<Mutex<HashMap<String, Vec<Instant>>>>,

    /// Backoff strategy
    backoff_strategy: BackoffStrategy,
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            timestamps: Arc::new(Mutex::new(HashMap::new())),
            backoff_strategy: BackoffStrategy::default(),
        }
    }

    /// Set the backoff strategy.
    pub fn with_backoff_strategy(mut self, strategy: BackoffStrategy) -> Self {
        self.backoff_strategy = strategy;
        self
    }

    /// Wait for rate limit to allow a request.
    pub async fn wait(&self, endpoint: &str) {
        // Get the current time
        let now = Instant::now();

        // Check if we need to wait
        let wait_time = {
            // Get the timestamps for this endpoint
            let mut timestamps = self.timestamps.lock().unwrap();
            let endpoint_timestamps = timestamps.entry(endpoint.to_string()).or_insert_with(Vec::new);

            // Remove timestamps older than 1 minute
            endpoint_timestamps.retain(|t| now.duration_since(*t) < Duration::from_secs(60));

            // Check if we've exceeded the rate limit
            if endpoint_timestamps.len() >= self.requests_per_minute as usize {
                // Calculate how long to wait
                let oldest = endpoint_timestamps[0];
                Some(Duration::from_secs(60) - now.duration_since(oldest))
            } else {
                // Add the current timestamp
                endpoint_timestamps.push(now);
                None
            }
        };

        // Wait if necessary
        if let Some(duration) = wait_time {
            // Wait for the rate limit to reset
            sleep(duration).await;

            // Add the current timestamp
            let mut timestamps = self.timestamps.lock().unwrap();
            let endpoint_timestamps = timestamps.entry(endpoint.to_string()).or_insert_with(Vec::new);
            endpoint_timestamps.push(Instant::now());
        }
    }

    /// Handle a rate limit error.
    pub async fn handle_rate_limit_error(&self, attempt: u32) -> Duration {
        self.backoff_strategy.get_backoff_duration(attempt)
    }
}

/// Backoff strategy for rate limiting.
#[derive(Debug, Clone, Copy)]
pub enum BackoffStrategy {
    /// Constant backoff
    Constant(Duration),

    /// Linear backoff
    Linear {
        /// Initial backoff duration
        initial: Duration,

        /// Increment per attempt
        increment: Duration,
    },

    /// Exponential backoff
    Exponential {
        /// Initial backoff duration
        initial: Duration,

        /// Multiplier per attempt
        multiplier: f64,

        /// Maximum backoff duration
        max: Duration,
    },
}

impl BackoffStrategy {
    /// Get the backoff duration for an attempt.
    pub fn get_backoff_duration(&self, attempt: u32) -> Duration {
        match self {
            Self::Constant(duration) => *duration,
            Self::Linear { initial, increment } => {
                *initial + *increment * attempt
            }
            Self::Exponential { initial, multiplier, max } => {
                let duration = initial.as_secs_f64() * multiplier.powf(attempt as f64);
                Duration::from_secs_f64(duration.min(max.as_secs_f64()))
            }
        }
    }
}

impl Default for BackoffStrategy {
    fn default() -> Self {
        Self::Exponential {
            initial: Duration::from_secs(1),
            multiplier: 2.0,
            max: Duration::from_secs(60),
        }
    }
}
