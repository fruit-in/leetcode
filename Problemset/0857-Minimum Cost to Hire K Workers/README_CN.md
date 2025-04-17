# 857. 雇佣 K 名工人的最低成本
有 `n` 名工人。 给定两个数组 `quality` 和 `wage` ，其中，`quality[i]` 表示第 `i` 名工人的工作质量，其最低期望工资为 `wage[i]` 。

现在我们想雇佣 `k` 名工人组成一个 **工资组**。在雇佣 一组 `k` 名工人时，我们必须按照下述规则向他们支付工资：
1. 对工资组中的每名工人，应当按其工作质量与同组其他工人的工作质量的比例来支付工资。
2. 工资组中的每名工人至少应当得到他们的最低期望工资。

给定整数 `k` ，返回 *组成满足上述条件的付费群体所需的最小金额* 。与实际答案误差相差在 <code>10<sup>-5</sup></code> 以内的答案将被接受。

#### 示例 1:
<pre>
<strong>输入:</strong> quality = [10,20,5], wage = [70,50,30], k = 2
<strong>输出:</strong> 105.00000
<strong>解释:</strong> 我们向 0 号工人支付 70，向 2 号工人支付 35。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
<strong>输出:</strong> 30.66667
<strong>解释:</strong> 我们向 0 号工人支付 4，向 2 号和 3 号分别支付 13.33333。
</pre>

#### 提示:
* `n == quality.length == wage.length`
* <code>1 <= k <= n <= 10<sup>4</sup></code>
* <code>1 <= quality[i], wage[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
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
