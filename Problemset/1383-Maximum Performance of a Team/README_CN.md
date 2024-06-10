# 1383. 最大的团队表现值
给定两个整数 `n` 和 `k`，以及两个长度为 `n` 的整数数组 `speed` 和 `efficiency`。现有 `n` 名工程师，编号从 `1` 到 `n`。其中 `speed[i]` 和 `efficiency[i]` 分别代表第 `i` 位工程师的速度和效率。

从这 `n` 名工程师中最多选择 `k` 名不同的工程师，使其组成的团队具有最大的团队表现值。

团队表现值 的定义为：一个团队中「所有工程师速度的和」乘以他们「效率值中的最小值」。

请你返回该团队的最大团队表现值，由于答案可能很大，请你返回结果对 `10^9 + 7` 取余后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
<strong>输出:</strong> 60
<strong>解释:</strong>
我们选择工程师 2（speed=10 且 efficiency=4）和工程师 5（speed=5 且 efficiency=7）。他们的团队表现值为 performance = (10 + 5) * min(4, 7) = 60 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 3
<strong>输出:</strong> 68
<strong>解释:</strong>
此示例与第一个示例相同，除了 k = 3 。我们可以选择工程师 1 ，工程师 2 和工程师 5 得到最大的团队表现值。表现值为 performance = (2 + 10 + 5) * min(5, 4, 7) = 68 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 4
<strong>输出:</strong> 72
</pre>

#### 提示:
* <code>1 <= k <= n <= 10<sup>5</sup></code>
* `speed.length == n`
* `efficiency.length == n`
* <code>1 <= speed[i] <= 10<sup>5</sup></code>
* <code>1 <= efficiency[i] <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
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
