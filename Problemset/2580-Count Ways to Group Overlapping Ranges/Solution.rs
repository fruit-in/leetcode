impl Solution {
    pub fn count_ways(mut ranges: Vec<Vec<i32>>) -> i32 {
        let mut max_end = -1;
        let mut ret = 1;

        ranges.sort_unstable();

        for i in 0..ranges.len() {
            if ranges[i][0] > max_end {
                ret = (ret * 2) % 1_000_000_007;
            }
            max_end = max_end.max(ranges[i][1]);
        }

        ret
    }
}
