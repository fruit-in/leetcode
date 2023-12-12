impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut stack = vec![];

        intervals.sort_unstable();

        for i in 0..intervals.len() {
            let (start0, end0) = (intervals[i][0], intervals[i][1]);
            let &(start1, end1) = stack.last().unwrap_or(&(0, start0));

            if start0 >= end1 {
                stack.push((start0, end0));
            } else if end0 <= end1 {
                stack.pop();
                stack.push((start0, end0));
            }
        }

        (intervals.len() - stack.len()) as i32
    }
}
