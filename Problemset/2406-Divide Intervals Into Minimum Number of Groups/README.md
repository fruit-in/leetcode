# 2406. Divide Intervals Into Minimum Number of Groups
You are given a 2D integer array `intervals` where <code>intervals[i] = [left<sub>i</sub>, right<sub>i</sub>]</code> represents the **inclusive** interval <code>[left<sub>i</sub>, right<sub>i</sub>]</code>.

You have to divide the intervals into one or more **groups** such that each interval is in **exactly** one group, and no two intervals that are in the same group **intersect** each other.

Return *the **minimum** number of groups you need to make*.

Two intervals **intersect** if there is at least one common number between them. For example, the intervals `[1, 5]` and `[5, 8]` intersect.

#### Example 1:
<pre>
<strong>Input:</strong> intervals = [[5,10],[6,8],[1,5],[2,3],[1,10]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can divide the intervals into the following groups:
- Group 1: [1, 5], [6, 8].
- Group 2: [2, 3], [5, 10].
- Group 3: [1, 10].
It can be proven that it is not possible to divide the intervals into fewer than 3 groups.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> intervals = [[1,3],[5,6],[8,10],[11,13]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> None of the intervals overlap, so we can put all of them in one group.
</pre>

#### Constraints:
* <code>1 <= intervals.length <= 10<sup>5</sup></code>
* `intervals[i].length == 2`
* <code>1 <= left<sub>i</sub> <= right<sub>i</sub> <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_groups(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut groups = BinaryHeap::new();

        intervals.sort_unstable();

        for interval in &intervals {
            let (left, right) = (interval[0], interval[1]);

            if *groups.peek().unwrap_or(&Reverse(i32::MAX)) > Reverse(left) {
                groups.pop();
            }
            groups.push(Reverse(right));
        }

        groups.len() as i32
    }
}
```
