impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events0 = events.iter().map(|e| (e[0], e[2])).collect::<Vec<_>>();
        let mut events1 = events.iter().map(|e| (e[1], e[2])).collect::<Vec<_>>();
        let mut max_end_value = 0;
        let mut i = 0;
        let mut ret = 0;

        events0.sort_unstable();
        events1.sort_unstable();

        for j in 0..events0.len() {
            while i < events1.len() && events1[i].0 < events0[j].0 {
                max_end_value = max_end_value.max(events1[i].1);
                i += 1;
            }

            ret = ret.max(events0[j].1 + max_end_value);
        }

        ret
    }
}
