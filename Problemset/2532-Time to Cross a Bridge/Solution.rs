use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut right_workers = BinaryHeap::new();
        let mut pick_workers = BinaryHeap::new();
        let mut left_workers = BinaryHeap::new();
        let mut put_workers = BinaryHeap::new();
        let mut pick = 0;
        let mut put = 0;
        let mut ret = 0;

        for i in 0..time.len() {
            left_workers.push((time[i][2] + time[i][0], i));
        }

        while put < n {
            while pick_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = pick_workers.pop().unwrap().1;
                right_workers.push((time[i][2] + time[i][0], i));
            }
            while put < n && put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = put_workers.pop().unwrap().1;
                left_workers.push((time[i][2] + time[i][0], i));
            }

            if let Some((_, i)) = right_workers.pop() {
                ret += time[i][2];
                put_workers.push((Reverse(ret + time[i][3]), i));
                put += 1;
            } else if pick == n || left_workers.is_empty() {
                ret = pick_workers
                    .peek()
                    .unwrap_or(&(Reverse(i32::MAX), 0))
                    .0
                     .0
                    .min(put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0);
            } else if let Some((_, i)) = left_workers.pop() {
                ret += time[i][0];
                pick_workers.push((Reverse(ret + time[i][1]), i));
                pick += 1;
            }
        }

        ret
    }
}
