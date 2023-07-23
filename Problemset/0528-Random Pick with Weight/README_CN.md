# 528. 按权重随机选择
给定一个正整数数组 `w` ，其中 `w[i]` 代表下标 `i` 的权重（下标从 `0` 开始），请写一个函数 `pickIndex` ，它可以随机地获取下标 `i`，选取下标 `i` 的概率与 `w[i]` 成正比。

例如，对于 `w = [1, 3]`，挑选下标 `0` 的概率为 `1 / (1 + 3) = 0.25` （即，25%），而选取下标 `1` 的概率为 `3 / (1 + 3) = 0.75`（即，75%）。

也就是说，选取下标 `i` 的概率为 `w[i] / sum(w)` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["Solution","pickIndex"]
[[[1]],[]]
<strong>输出:</strong>
[null,0]
<strong>解释:</strong>
Solution solution = new Solution([1]);
solution.pickIndex(); // 返回 0，因为数组中只有一个元素，所以唯一的选择是返回下标 0。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
[[[1,3]],[],[],[],[],[]]
<strong>输出:</strong>
[null,1,1,1,1,0]
<strong>解释:</strong>
Solution solution = new Solution([1, 3]);
solution.pickIndex(); // 返回 1，返回下标 1，返回该下标概率为 3/4 。
solution.pickIndex(); // 返回 1
solution.pickIndex(); // 返回 1
solution.pickIndex(); // 返回 1
solution.pickIndex(); // 返回 0，返回下标 0，返回该下标概率为 1/4 。

由于这是一个随机问题，允许多个答案，因此下列输出都可以被认为是正确的:
[null,1,1,1,1,0]
[null,1,1,1,1,1]
[null,1,1,1,0,0]
[null,1,1,1,0,1]
[null,1,0,1,0,0]
......
诸若此类。
</pre>

#### 提示:
* `1 <= w.length <= 10000`
* `1 <= w[i] <= 10^5`
* `pickIndex` 将被调用不超过 `10000` 次

## 题解 (Rust)

### 1. 二分查找
```Rust
use rand::{thread_rng, Rng};

struct Solution {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }

        Self { prefix_sum: w }
    }

    fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1, self.prefix_sum.last().unwrap() + 1);

        match self.prefix_sum.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
```
