use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let target_arrival = times[target_friend as usize][0];
        let mut chair_inf = 0;
        let mut chair_heap = BinaryHeap::new();
        let mut leaving_heap = BinaryHeap::new();
        let mut times = times;
        times.sort_unstable();

        for time in &times {
            while -leaving_heap.peek().unwrap_or(&(-100001, 0)).0 <= time[0] {
                chair_heap.push(leaving_heap.pop().unwrap().1);
            }

            match chair_heap.pop() {
                Some(chair) if time[0] == target_arrival => return -chair,
                Some(chair) => leaving_heap.push((-time[1], chair)),
                None if time[0] == target_arrival => return chair_inf,
                None => {
                    leaving_heap.push((-time[1], -chair_inf));
                    chair_inf += 1;
                }
            }
        }

        unreachable!()
    }
}
