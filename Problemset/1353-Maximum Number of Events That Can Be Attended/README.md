# 1353. Maximum Number of Events That Can Be Attended
You are given an array of `events` where <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>]</code>. Every event `i` starts at <code>startDay<sub>i</sub></code> and ends at <code>endDay<sub>i</sub></code>.

You can attend an event `i` at any day `d` where <code>startTime<sub>i</sub> <= d <= endTime<sub>i</sub></code>. You can only attend one event at any time `d`.

Return *the maximum number of events you can attend*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/05/e1.png)
<pre>
<strong>Input:</strong> events = [[1,2],[2,3],[3,4]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can attend all the three events.
One way to attend them all is as shown.
Attend the first event on day 1.
Attend the second event on day 2.
Attend the third event on day 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> events= [[1,2],[2,3],[3,4],[1,2]]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* <code>1 <= events.length <= 105</sup></code>
* `events[i].length == 2`
* <code>1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 105</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        let mut heap = BinaryHeap::new();
        let first_day = events.iter().map(|e| e[0]).min().unwrap();
        let final_day = events.iter().map(|e| e[1]).max().unwrap();
        let mut i = 0;
        let mut ret = 0;

        events.sort_unstable();

        for day in first_day..=final_day {
            while i < events.len() && events[i][0] <= day {
                heap.push(-events[i][1]);
                i += 1;
            }
            while let Some(end_day) = heap.pop() {
                if -end_day >= day {
                    ret += 1;
                    break;
                }
            }
        }

        ret
    }
}
```
