use std::collections::BinaryHeap;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        let mut heap = BinaryHeap::new();
        let first_day = events.iter().map(|e| e[0]).min().unwrap();
        let final_day = events.iter().map(|e| e[1]).max().unwrap();
        let mut i = 0;
        let mut ret = 0;

        events.sort_unstable();

        for day in first_day..=final_day {
            while i < events.len() && events[i][0] <= day {
                heap.push(-events[i][1]);
                i += 1;
            }
            while let Some(end_day) = heap.pop() {
                if -end_day >= day {
                    ret += 1;
                    break;
                }
            }
        }

        ret
    }
}
