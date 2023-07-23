use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut minutes = HashMap::new();
        let mut answer = vec![0; k as usize];

        for log in logs {
            minutes
                .entry(log[0])
                .or_insert(HashSet::new())
                .insert(log[1]);
        }

        for (id, min) in minutes {
            answer[min.len() - 1] += 1;
        }

        answer
    }
}
