impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = vec![];
        let mut prev = intervals.pop().unwrap();

        while let Some(curr) = intervals.pop() {
            if curr[0] > prev[1] {
                ret.push(prev);
                prev = curr;
            } else {
                prev[1] = prev[1].max(curr[1]);
            }
        }
        ret.push(prev);

        ret
    }
}
