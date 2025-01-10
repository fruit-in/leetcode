# 1439. 有序矩阵中的第 k 个最小数组和
给你一个 `m * n` 的矩阵 `mat`，以及一个整数 `k` ，矩阵中的每一行都以非递减的顺序排列。

你可以从每一行中选出 1 个元素形成一个数组。返回所有可能数组中的第 k 个 **最小** 数组和。

#### 示例 1:
<pre>
<strong>输入:</strong> mat = [[1,3,11],[2,4,6]], k = 5
<strong>输出:</strong> 7
<strong>解释:</strong> 从每一行中选出一个元素，前 k 个和最小的数组分别是：
[1,2], [1,4], [3,2], [3,4], [1,6]。其中第 5 个的和是 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> mat = [[1,3,11],[2,4,6]], k = 9
<strong>输出:</strong> 17
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> mat = [[1,10,10],[1,4,5],[2,3,6]], k = 7
<strong>输出:</strong> 9
<strong>解释:</strong> 从每一行中选出一个元素，前 k 个和最小的数组分别是：
[1,1,2], [1,1,3], [1,4,2], [1,4,3], [1,1,6], [1,5,2], [1,5,3]。其中第 7 个的和是 9 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> mat = [[1,1,10],[2,2,9]], k = 7
<strong>输出:</strong> 12
</pre>

#### 提示:
* `m == mat.length`
* `n == mat.length[i]`
* `1 <= m, n <= 40`
* <code>1 <= k <= min(200, n<sup>m</sup>)</code>
* `1 <= mat[i][j] <= 5000`
* `mat[i]` 是一个非递减数组

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from([0]);

        for i in 0..mat.len() {
            let mut tmp = BinaryHeap::new();

            while let Some(x) = heap.pop() {
                for j in 0..mat[0].len() {
                    tmp.push(x + mat[i][j]);

                    if tmp.len() as i32 > k {
                        tmp.pop();
                    }
                }
            }

            heap = tmp;
        }

        heap.pop().unwrap()
    }
}
```
