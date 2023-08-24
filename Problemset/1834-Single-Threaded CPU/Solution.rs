use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut time = 0;
        let mut tasks = tasks.iter().enumerate().collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        tasks.sort_unstable_by_key(|(_, t)| -t[0]);

        while !tasks.is_empty() || !heap.is_empty() {
            while let Some(&(i, t)) = tasks.last() {
                if t[0] as i64 <= time {
                    heap.push(Reverse((t[1], i)));
                    tasks.pop();
                } else {
                    break;
                }
            }

            match heap.pop() {
                Some(Reverse((pt, i))) => {
                    time += pt as i64;
                    ret.push(i as i32);
                }
                None => time = tasks.last().unwrap().1[0] as i64,
            }
        }

        ret
    }
}
