# 2054. 两个最好的不重叠活动
给你一个下标从 **0** 开始的二维整数数组 `events` ，其中 <code>events[i] = [startTime<sub>i</sub>, endTime<sub>i</sub>, value<sub>i</sub>]</code> 。第 `i` 个活动开始于 <code>startTime<sub>i</sub></code> ，结束于 <code>endTime<sub>i</sub></code> ，如果你参加这个活动，那么你可以得到价值 <code>value<sub>i</sub></code> 。你 **最多** 可以参加 **两个时间不重叠** 活动，使得它们的价值之和 **最大** 。

请你返回价值之和的 **最大值** 。

注意，活动的开始时间和结束时间是 **包括** 在活动时间内的，也就是说，你不能参加两个活动且它们之一的开始时间等于另一个活动的结束时间。更具体的，如果你参加一个活动，且结束时间为 `t` ，那么下一个活动必须在 `t + 1` 或之后的时间开始。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/21/picture5.png)
<pre>
<strong>输入:</strong> events = [[1,3,2],[4,5,2],[2,4,3]]
<strong>输出:</strong> 4
<strong>解释:</strong> 选择绿色的活动 0 和 1 ，价值之和为 2 + 2 = 4 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/21/picture1.png)
<pre>
<strong>输入:</strong> events = [[1,3,2],[4,5,2],[1,5,5]]
<strong>输出:</strong> 5
<strong>解释:</strong> 选择活动 2 ，价值和为 5 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/09/21/picture3.png)
<pre>
<strong>输入:</strong> events = [[1,5,3],[1,5,1],[6,6,5]]
<strong>输出:</strong> 8
<strong>解释:</strong> 选择活动 0 和 2 ，价值之和为 3 + 5 = 8 。
</pre>

#### 提示:
* <code>2 <= events.length <= 105</sup></code>
* `events[i].length == 3`
* <code>1 <= startTime<sub>i</sub> <= endTime<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= value<sub>i</sub> <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
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
