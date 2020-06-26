# 56. Merge Intervals
Given a collection of intervals, merge all overlapping intervals.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,3],[2,6],[8,10],[15,18]]
<strong>Output:</strong> [[1,6],[8,10],[15,18]]
<strong>Explanation:</strong> Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,4],[4,5]]
<strong>Output:</strong> [[1,5]]
<strong>Explanation:</strong> Intervals [1,4] and [4,5] are considered overlapping.
</pre>

**NOTE:** input types have been changed on April 15, 2019. Please reset to default code definition to get new method signature.

## Solutions (Rust)

### 1. Sort
```Rust
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
```
