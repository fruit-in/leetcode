impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut end = -1;
        let mut ret = 0;

        for i in 0..time_series.len() {
            if time_series[i] >= end {
                ret += duration;
            } else {
                ret += time_series[i] - time_series[i - 1];
            }
            end = time_series[i] + duration;
        }

        ret
    }
}
