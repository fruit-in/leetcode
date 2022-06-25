impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        operations.iter().map(|o| 44 - o.as_bytes()[1] as i32).sum()
    }
}
