# 1383. Maximum Performance of a Team
You are given two integers `n` and `k` and two integer arrays `speed` and `efficiency` both of length `n`. There are `n` engineers numbered from `1` to `n`. `speed[i]` and `efficiency[i]` represent the speed and efficiency of the <code>i<sup>th</sup></code> engineer respectively.

Choose **at most** `k` different engineers out of the `n` engineers to form a team with the maximum **performance**.

The performance of a team is the sum of its engineers' speeds multiplied by the minimum efficiency among its engineers.

Return *the maximum performance of this team*. Since the answer can be a huge number, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
<strong>Output:</strong> 60
<strong>Explanation:</strong>
We have the maximum performance of the team by selecting engineer 2 (with speed=10 and efficiency=4) and engineer 5 (with speed=5 and efficiency=7). That is, performance = (10 + 5) * min(4, 7) = 60.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 3
<strong>Output:</strong> 68
<strong>Explanation:</strong>
This is the same example as the first but k = 3. We can select engineer 1, engineer 2 and engineer 5 to get the maximum performance of the team. That is, performance = (2 + 10 + 5) * min(5, 4, 7) = 68.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 4
<strong>Output:</strong> 72
</pre>

#### Constraints:
* <code>1 <= k <= n <= 10<sup>5</sup></code>
* `speed.length == n`
* `efficiency.length == n`
* <code>1 <= speed[i] <= 10<sup>5</sup></code>
* <code>1 <= efficiency[i] <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut engineers = efficiency.iter().zip(speed.iter()).collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut speed_sum = 0;
        let mut ret = 0;

        engineers.sort_unstable();

        for &(&e, &s) in engineers.iter().rev() {
            if heap.len() == k as usize {
                speed_sum += heap.pop().unwrap();
            }
            speed_sum += s as i64;
            heap.push(-s as i64);
            ret = ret.max(speed_sum * e as i64);
        }

        (ret % 1_000_000_007) as i32
    }
}
```
