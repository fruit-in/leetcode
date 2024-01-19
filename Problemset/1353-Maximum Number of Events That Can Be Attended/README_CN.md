# 1353. 最多可以参加的会议数目
给你一个数组 `events`，其中 <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>]</code> ，表示会议 `i` 开始于 <code>startDay<sub>i</sub></code> ，结束于 <code>endDay<sub>i</sub></code> 。

你可以在满足 <code>startDay<sub>i</sub> <= d <= endDay<sub>i</sub></code> 中的任意一天 `d` 参加会议 `i` 。注意，一天只能参加一个会议。

请你返回你可以参加的 **最大** 会议数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/02/05/e1.png)
<pre>
<strong>输入:</strong> events = [[1,2],[2,3],[3,4]]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以参加所有的三个会议。
安排会议的一种方案如上图。
第 1 天参加第一个会议。
第 2 天参加第二个会议。
第 3 天参加第三个会议。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> events= [[1,2],[2,3],[3,4],[1,2]]
<strong>输出:</strong> 4
</pre>

#### 提示:
* <code>1 <= events.length <= 105</sup></code>
* `events[i].length == 2`
* <code>1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 105</sup></code>

## 题解 (Rust)

### 1. 题解
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
