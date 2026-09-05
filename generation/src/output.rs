//! Generation output and statistics.

use std::time::Duration;

/// Result of a generation request.
#[derive(Debug)]
pub struct GenerationOutput {
    /// Generated source code.
    pub code: String,

    /// Generation statistics.
    pub statistics: GenerationStatistics,
}

/// Statistics collected during generation.
#[derive(Debug)]
pub struct GenerationStatistics {
    /// Number of prompt tokens, when reported.
    pub prompt_tokens: Option<u64>,

    /// Number of generated tokens.
    pub generated_tokens: u64,

    /// Total request duration.
    pub elapsed: Duration,
}

impl GenerationStatistics {
    /// Returns observed generation throughput in tokens per second.
    pub fn tokens_per_second(&self) -> f64 {
        let seconds = self.elapsed.as_secs_f64();

        if seconds == 0.0 {
            return 0.0;
        }

        self.generated_tokens as f64 / seconds
    }
}
