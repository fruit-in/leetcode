# 2054. Two Best Non-Overlapping Events
You are given a **0-indexed** 2D integer array of `events` where <code>events[i] = [startTime<sub>i</sub>, endTime<sub>i</sub>, value<sub>i</sub>]</code>. The <code>i<sup>th</sup></code> event starts at <code>startTime<sub>i</sub></code> and ends at <code>endTime<sub>i</sub></code>, and if you attend this event, you will receive a value of <code>value<sub>i</sub></code>. You can choose **at most two non-overlapping** events to attend such that the sum of their values is **maximized**.

Return *this **maximum** sum*.

Note that the start time and end time is **inclusive**: that is, you cannot attend two events where one of them starts and the other ends at the same time. More specifically, if you attend an event with end time `t`, the next event must start at or after `t + 1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/09/21/picture5.png)
<pre>
<strong>Input:</strong> events = [[1,3,2],[4,5,2],[2,4,3]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Choose the green events, 0 and 1 for a sum of 2 + 2 = 4.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/09/21/picture1.png)
<pre>
<strong>Input:</strong> events = [[1,3,2],[4,5,2],[1,5,5]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> Choose event 2 for a sum of 5.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/09/21/picture3.png)
<pre>
<strong>Input:</strong> events = [[1,5,3],[1,5,1],[6,6,5]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> Choose events 0 and 2 for a sum of 3 + 5 = 8.
</pre>

#### Constraints:
* <code>2 <= events.length <= 105</sup></code>
* `events[i].length == 3`
* <code>1 <= startTime<sub>i</sub> <= endTime<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= value<sub>i</sub> <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events0 = events.iter().map(|e| (e[0], e[2])).collect::<Vec<_>>();
        let mut events1 = events.iter().map(|e| (e[1], e[2])).collect::<Vec<_>>();
        let mut max_end_value = 0;
        let mut i = 0;
        let mut ret = 0;

        events0.sort_unstable();
        events1.sort_unstable();

        for j in 0..events0.len() {
            while i < events1.len() && events1[i].0 < events0[j].0 {
                max_end_value = max_end_value.max(events1[i].1);
                i += 1;
            }

            ret = ret.max(events0[j].1 + max_end_value);
        }

        ret
    }
}
```
