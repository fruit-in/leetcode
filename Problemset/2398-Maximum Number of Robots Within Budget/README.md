# 2398. Maximum Number of Robots Within Budget
You have `n` robots. You are given two **0-indexed** integer arrays, `chargeTimes` and `runningCosts`, both of length `n`. The <code>i<sup>th</sup></code> robot costs `chargeTimes[i]` units to charge and costs `runningCosts[i]` units to run. You are also given an integer `budget`.

The **total cost** of running `k` chosen robots is equal to `max(chargeTimes) + k * sum(runningCosts)`, where `max(chargeTimes)` is the largest charge cost among the `k` robots and `sum(runningCosts)` is the sum of running costs among the `k` robots.

Return *the **maximum** number of **consecutive** robots you can run such that the total cost **does not** exceed* `budget`.

#### Example 1:
<pre>
<strong>Input:</strong> chargeTimes = [3,6,1,3,4], runningCosts = [2,1,3,4,5], budget = 25
<strong>Output:</strong> 3
<strong>Explanation:</strong>
It is possible to run all individual and consecutive pairs of robots within budget.
To obtain answer 3, consider the first 3 robots. The total cost will be max(3,6,1) + 3 * sum(2,1,3) = 6 + 3 * 6 = 24 which is less than 25.
It can be shown that it is not possible to run more than 3 consecutive robots within budget, so we return 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> chargeTimes = [11,12,19], runningCosts = [10,8,7], budget = 19
<strong>Output:</strong> 0
<strong>Explanation:</strong> No robot can be run that does not exceed the budget, so we return 0.
</pre>

#### Constraints:
* `chargeTimes.length == runningCosts.length == n`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= chargeTimes[i], runningCosts[i] <= 10<sup>5</sup></code>
* <code>1 <= budget <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
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
