pub mod entrypoint;
pub mod requester;
pub mod util;

/// The concurrency limits set for the concurrent requests
pub const CONCURRENCY_LIMIT: usize = 10;
