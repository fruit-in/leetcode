# 502. IPO
Suppose LeetCode will start its **IPO** soon. In order to sell a good price of its shares to Venture Capital, LeetCode would like to work on some projects to increase its capital before the **IPO**. Since it has limited resources, it can only finish at most `k` distinct projects before the **IPO**. Help LeetCode design the best way to maximize its total capital after finishing at most `k` distinct projects.

You are given `n` projects where the <code>i<sup>th</sup></code> project has a pure profit `profits[i]` and a minimum capital of `capital[i]` is needed to start it.

Initially, you have `w` capital. When you finish a project, you will obtain its pure profit and the profit will be added to your total capital.

Pick a list of **at most** `k` distinct projects from given projects to **maximize your final capital**, and return *the final maximized capital*.

The answer is guaranteed to fit in a 32-bit signed integer.

#### Example 1:
<pre>
<strong>Input:</strong> k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Since your initial capital is 0, you can only start the project indexed 0.
After finishing it you will obtain profit 1 and your capital becomes 1.
With capital 1, you can either start the project indexed 1 or the project indexed 2.
Since you can choose at most 2 projects, you need to finish the project indexed 2 to get the maximum capital.
Therefore, output the final maximized capital, which is 0 + 1 + 3 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
<strong>Output:</strong> 6
</pre>

#### Constraints:
* <code>1 <= k <= 10<sup>5</sup></code>
* <code>0 <= w <= 10<sup>9</sup></code>
* `n == profits.length`
* `n == capital.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= profits[i] <= 10<sup>4</sup></code>
* <code>0 <= capital[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut projects = capital
            .into_iter()
            .zip(profits.into_iter())
            .collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut i = 0;

        projects.sort_unstable();

        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= w {
                heap.push(projects[i].1);
                i += 1;
            }

            w += heap.pop().unwrap_or(0);
        }

        w
    }
}
```
