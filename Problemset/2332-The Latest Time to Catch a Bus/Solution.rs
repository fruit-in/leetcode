use std::collections::HashSet;

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        let passengers_set = passengers.clone().into_iter().collect::<HashSet<_>>();
        let mut i = 0;
        let mut ret = 1;

        buses.sort_unstable();
        passengers.sort_unstable();

        for j in 0..buses.len() {
            let mut count = 0;
            let mut time = buses[j];

            while i < passengers.len() && passengers[i] <= buses[j] && count < capacity {
                count += 1;
                i += 1;
            }

            if count == capacity {
                time = passengers[i - 1];
            }

            while passengers_set.contains(&time) {
                time -= 1;
            }

            ret = ret.max(time);
        }

        ret
    }
}
