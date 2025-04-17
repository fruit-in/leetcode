# 857. Minimum Cost to Hire K Workers
There are `n` workers. You are given two integer arrays `quality` and `wage` where `quality[i]` is the quality of the <code>i<sup>th</sup></code> worker and `wage[i]` is the minimum wage expectation for the <code>i<sup>th</sup></code> worker.

We want to hire exactly `k` workers to form a **paid group**. To hire a group of `k` workers, we must pay them according to the following rules:
1. Every worker in the paid group must be paid at least their minimum wage expectation.
2. In the group, each worker's pay must be directly proportional to their quality. This means if a worker’s quality is double that of another worker in the group, then they must be paid twice as much as the other worker.

Given the integer `k`, return *the least amount of money needed to form a paid group satisfying the above conditions*. Answers within <code>10<sup>-5</sup></code> of the actual answer will be accepted.

#### Example 1:
<pre>
<strong>Input:</strong> quality = [10,20,5], wage = [70,50,30], k = 2
<strong>Output:</strong> 105.00000
<strong>Explanation:</strong> We pay 70 to 0th worker and 35 to 2nd worker.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
<strong>Output:</strong> 30.66667
<strong>Explanation:</strong> We pay 4 to 0th worker, 13.33333 to 2nd and 3rd workers separately.
</pre>

#### Constraints:
* `n == quality.length == wage.length`
* <code>1 <= k <= n <= 10<sup>4</sup></code>
* <code>1 <= quality[i], wage[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut workers = quality
            .into_iter()
            .zip(wage.into_iter())
            .collect::<Vec<_>>();
        let mut quality_heap = BinaryHeap::new();
        let mut quality_sum = 0;
        let mut ret = f64::INFINITY;

        workers.sort_unstable_by(|(q0, w0), (q1, w1)| (q1 * w0).cmp(&(q0 * w1)));

        for i in 0..workers.len() {
            if i >= k {
                quality_sum -= quality_heap.pop().unwrap();
            }

            quality_heap.push(workers[i].0);
            quality_sum += workers[i].0;

            if i >= k - 1 {
                ret = ret.min(quality_sum as f64 * workers[i].1 as f64 / workers[i].0 as f64);
            }
        }

        ret
    }
}
```
