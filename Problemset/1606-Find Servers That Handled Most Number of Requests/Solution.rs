use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;
        let mut free = (0..k).map(|i| Reverse(i)).collect::<BinaryHeap<_>>();
        let mut busy = BinaryHeap::new();
        let mut count = vec![0; load.len()];

        for i in 0..load.len() {
            while let Some(&(Reverse(t), j)) = busy.peek() {
                if t > arrival[i] {
                    break;
                } else if i % k > j {
                    free.push(Reverse(j + i / k * k + k));
                } else {
                    free.push(Reverse(j + i / k * k));
                }
                busy.pop();
            }

            if let Some(Reverse(j)) = free.pop() {
                busy.push((Reverse(arrival[i] + load[i]), j % k));
                count[j % k] += 1;
            }
        }

        let max_count = *count.iter().max().unwrap();

        (0..k as i32)
            .filter(|&i| *count.get(i as usize).unwrap_or(&0) == max_count)
            .collect()
    }
}
