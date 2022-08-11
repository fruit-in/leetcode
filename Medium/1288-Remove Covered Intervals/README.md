# 1288. Remove Covered Intervals
Given an array `intervals` where `intervals[i] = [li, ri]` represent the interval `[li, ri)`, remove all intervals that are covered by another interval in the list.

The interval `[a, b)` is covered by the interval `[c, d)` if and only if `c <= a` and `b <= d`.

Return *the number of remaining intervals*.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[1,4],[3,6],[2,8]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Interval [3,6] is covered by [2,8], therefore it is removed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,4],[2,3]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= intervals.length <= 1000`
* `intervals[i].length == 2`
* <code>0 <= li < ri <= 10<sup>5</sup></code>
* All the given intervals are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
