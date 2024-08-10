# 2398. 预算内的最多机器人数目
你有 `n` 个机器人，给你两个下标从 **0** 开始的整数数组 `chargeTimes` 和 `runningCosts` ，两者长度都为 `n` 。第 `i` 个机器人充电时间为 `chargeTimes[i]` 单位时间，花费 `runningCosts[i]` 单位时间运行。再给你一个整数 `budget` 。

运行 `k` 个机器人 **总开销** 是 `max(chargeTimes) + k * sum(runningCosts)` ，其中 `max(chargeTimes)` 是这 `k` 个机器人中最大充电时间，`sum(runningCosts)` 是这 `k` 个机器人的运行时间之和。

请你返回在 **不超过** `budget` 的前提下，你 **最多** 可以 **连续** 运行的机器人数目为多少。

#### 示例 1:
<pre>
<strong>输入:</strong> chargeTimes = [3,6,1,3,4], runningCosts = [2,1,3,4,5], budget = 25
<strong>输出:</strong> 3
<strong>解释:</strong>
可以在 budget 以内运行所有单个机器人或者连续运行 2 个机器人。
选择前 3 个机器人，可以得到答案最大值 3 。总开销是 max(3,6,1) + 3 * sum(2,1,3) = 6 + 3 * 6 = 24 ，小于 25 。
可以看出无法在 budget 以内连续运行超过 3 个机器人，所以我们返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> chargeTimes = [11,12,19], runningCosts = [10,8,7], budget = 19
<strong>输出:</strong> 0
<strong>解释:</strong> 即使运行任何一个单个机器人，还是会超出 budget，所以我们返回 0 。
</pre>

#### 提示:
* `chargeTimes.length == runningCosts.length == n`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= chargeTimes[i], runningCosts[i] <= 10<sup>5</sup></code>
* <code>1 <= budget <= 10<sup>15</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut k = 0;
        let mut deque = VecDeque::new();
        let mut max_charge = 0;
        let mut running_sum = 0;
        let mut total_cost = 0;
        let mut ret = 0;

        for i in 0..charge_times.len() {
            k += 1;
            while deque.back().unwrap_or(&(0, i64::MAX)).1 <= charge_times[i] as i64 {
                deque.pop_back();
            }
            deque.push_back((i, charge_times[i] as i64));
            max_charge = deque.front().unwrap().1;
            running_sum += running_costs[i] as i64;
            total_cost = max_charge + k as i64 * running_sum;

            while total_cost > budget {
                k -= 1;
                if deque.front().unwrap().0 < i - k + 1 {
                    deque.pop_front();
                }
                max_charge = deque.front().unwrap_or(&(0, 0)).1;
                running_sum -= running_costs[i - k] as i64;
                total_cost = max_charge + k as i64 * running_sum;
            }

            ret = ret.max(k as i32);
        }

        ret
    }
}
```
