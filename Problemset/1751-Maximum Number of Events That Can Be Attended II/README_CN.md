# 1751. 最多可以参加的会议数目 II
给你一个 `events` 数组，其中 <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>, value<sub>i</sub>]</code> ，表示第 `i` 个会议在 <code>startDay<sub>i</sub></code> 天开始，第 <code>endDay<sub>i</sub></code> 天结束，如果你参加这个会议，你能得到价值 <code>value<sub>i</sub></code> 。同时给你一个整数 `k` 表示你能参加的最多会议数目。

你同一时间只能参加一个会议。如果你选择参加某个会议，那么你必须 **完整** 地参加完这个会议。会议结束日期是包含在会议内的，也就是说你不能同时参加一个开始日期与另一个结束日期相同的两个会议。

请你返回能得到的会议价值 **最大和** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png)
<pre>
<strong>输入:</strong> events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 选择绿色的活动会议 0 和 1，得到总价值和为 4 + 3 = 7 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png)
<pre>
<strong>输入:</strong> events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
<strong>输出:</strong> 10
<strong>解释:</strong> 参加会议 2 ，得到价值和为 10 。
你没法再参加别的会议了，因为跟会议 2 有重叠。你 不 需要参加满 k 个会议。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png)
<pre>
<strong>输入:</strong> events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
<strong>输出:</strong> 9
<strong>解释:</strong> 尽管会议互不重叠，你只能参加 3 个会议，所以选择价值最大的 3 个会议。
</pre>

#### 提示:
* `1 <= k <= events.length`
* <code>1 <= k * events.length <= 10<sup>6</sup></code>
* <code>1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= value<sub>i</sub> <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![[0, 0]]; k + 1];
        let mut ret = 0;

        events.sort_unstable_by_key(|e| e[1]);

        for i in 0..events.len() {
            let (start, end, value) = (events[i][0], events[i][1], events[i][2]);

            for j in (0..k.min(i + 1)).rev() {
                let value_sum =
                    dp[j][dp[j].binary_search(&[start - 1, i32::MAX]).unwrap_err() - 1][1] + value;

                if dp[j + 1].last().unwrap()[1] < value_sum {
                    if dp[j + 1].last().unwrap()[0] == end {
                        dp[j + 1].pop();
                    }
                    dp[j + 1].push([end, value_sum]);
                    ret = ret.max(value_sum);
                }
            }
        }

        ret
    }
}
```
