# 1751. Maximum Number of Events That Can Be Attended II
You are given an array of `events` where <code>events[i] = [startDay<sub>i</sub>, endDay<sub>i</sub>, value<sub>i</sub>]</code>. The <code>i<sup>th</sup></code> event starts at <code>startDay<sub>i</sub></code> and ends at <code>endDay<sub>i</sub></code>, and if you attend this event, you will receive a value of <code>value<sub>i</sub></code>. You are also given an integer `k` which represents the maximum number of events you can attend.

You can only attend one event at a time. If you choose to attend an event, you must attend the **entire** event. Note that the end day is **inclusive**: that is, you cannot attend two events where one of them starts and the other ends on the same day.

Return *the **maximum sum** of values that you can receive by attending events*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60048-pm.png)
<pre>
<strong>Input:</strong> events = [[1,2,4],[3,4,3],[2,3,1]], k = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> Choose the green events, 0 and 1 (0-indexed) for a total value of 4 + 3 = 7.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60150-pm.png)
<pre>
<strong>Input:</strong> events = [[1,2,4],[3,4,3],[2,3,10]], k = 2
<strong>Output:</strong> 10
<strong>Explanation:</strong> Choose event 2 for a total value of 10.
Notice that you cannot attend any other event as they overlap, and that you do not have to attend k events.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-60703-pm.png)
<pre>
<strong>Input:</strong> events = [[1,1,1],[2,2,2],[3,3,3],[4,4,4]], k = 3
<strong>Output:</strong> 9
<strong>Explanation:</strong> Although the events do not overlap, you can only attend 3 events. Pick the highest valued three.
</pre>

#### Constraints:
* `1 <= k <= events.length`
* <code>1 <= k * events.length <= 10<sup>6</sup></code>
* <code>1 <= startDay<sub>i</sub> <= endDay<sub>i</sub> <= 10<sup>9</sup></code>
* <code>1 <= value<sub>i</sub> <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
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
