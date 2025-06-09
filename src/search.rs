use std::time::Instant;
use crate::types::SearchResult;

pub fn sequential_search(arr: &[i32], target: i32) -> SearchResult {
    let start = Instant::now();
    let mut positions_visited = 0;
    
    for &value in arr.iter() {
        positions_visited += 1;
        if value == target {
            let duration = start.elapsed().as_nanos();
            return SearchResult {
                positions_visited,
                execution_time_ns: duration,
            };
        }
    }
    
    let duration = start.elapsed().as_nanos();
    SearchResult {
        positions_visited,
        execution_time_ns: duration,
    }
}

pub fn binary_search(arr: &[i32], target: i32) -> SearchResult {
    let start = Instant::now();
    let mut positions_visited = 0;
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        positions_visited += 1;
        let mid = left + (right - left) / 2;
        
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => {
                let duration = start.elapsed().as_nanos();
                return SearchResult {
                    positions_visited,
                    execution_time_ns: duration,
                };
            }
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    
    let duration = start.elapsed().as_nanos();
    SearchResult {
        positions_visited,
        execution_time_ns: duration,
    }
}