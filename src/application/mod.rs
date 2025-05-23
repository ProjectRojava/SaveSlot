//! Application layer services

pub struct QueryService {}

impl QueryService {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for QueryService {
    fn default() -> Self {
        Self::new()
    }
}
