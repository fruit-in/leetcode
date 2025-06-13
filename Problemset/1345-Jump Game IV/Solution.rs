use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut js = HashMap::new();
        let mut visited = HashSet::from([0]);
        let mut deque = VecDeque::from([(0, 0)]);

        for i in 0..arr.len() {
            js.entry(arr[i]).or_insert(vec![]).push(i);
        }

        while let Some((i, steps)) = deque.pop_front() {
            if i == arr.len() - 1 {
                return steps;
            }

            if i > 0 && !visited.contains(&(i - 1)) {
                visited.insert(i - 1);
                deque.push_back((i - 1, steps + 1));
            }
            if i < arr.len() - 1 && !visited.contains(&(i + 1)) {
                visited.insert(i + 1);
                deque.push_back((i + 1, steps + 1));
            }
            for j in js.remove(&arr[i]).unwrap_or(vec![]) {
                if !visited.contains(&j) {
                    visited.insert(j);
                    deque.push_back((j, steps + 1));
                }
            }
        }

        unreachable!()
    }
}
