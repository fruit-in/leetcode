impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut starts = vec![];
        let mut ends = vec![];
        let mut i = 0;
        let mut ret = vec![-1; intervals.len()];

        for j in 0..intervals.len() {
            starts.push((intervals[j][0], j));
            ends.push((intervals[j][1], j));
        }
        starts.sort_unstable();
        ends.sort_unstable();

        for j in 0..ends.len() {
            while i < ends.len() && ends[j].0 > starts[i].0 {
                i += 1;
            }
            if i >= ends.len() {
                break;
            }
            ret[ends[j].1] = starts[i].1 as i32;
        }

        ret
    }
}
