#[derive(Debug)]
pub struct SearchResult {
    pub positions_visited: usize,
    pub execution_time_ns: u128,
}

#[derive(Debug)]
pub struct BenchmarkResults {
    pub size: usize,
    pub sequential_avg_case: SearchResult,
    pub sequential_worst_case: SearchResult,
    pub binary_avg_case: SearchResult,
    pub binary_worst_case: SearchResult,
}