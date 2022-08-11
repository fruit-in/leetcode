impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut max_r = 0;
        let mut ret = intervals.len() as i32;
        intervals.sort_unstable_by_key(|interval| (interval[0], -interval[1]));

        for interval in intervals {
            if interval[1] <= max_r {
                ret -= 1;
            } else {
                max_r = interval[1];
            }
        }

        ret
    }
}
