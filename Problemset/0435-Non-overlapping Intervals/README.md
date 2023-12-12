# 435. Non-overlapping Intervals
Given an array of intervals `intervals` where <code>intervals[i] = [start<sub>i</sub>, end<sub>i</sub>]</code>, return *the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping*.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,2],[2,3],[3,4],[1,3]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> [1,3] can be removed and the rest of the intervals are non-overlapping.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,2],[1,2],[1,2]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You need to remove two [1,2] to make the rest of the intervals non-overlapping.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> intervals = [[1,2],[2,3]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> You don't need to remove any of the intervals since they're already non-overlapping.
</pre>

#### Constraints:
* <code>1 <= intervals.length <= 105</sup></code>
* `intervals[i].length == 2`
* <code>-5 * 10<sup>4</sup> <= start<sub>i</sub> < end<sub>i</sub> <= 5 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
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
```
