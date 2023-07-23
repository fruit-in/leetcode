# 454. 四数相加 II
给定四个包含整数的数组列表 A , B , C , D ,计算有多少个元组 ```(i, j, k, l)``` ，使得 ```A[i] + B[j] + C[k] + D[l] = 0```。

为了使问题简单化，所有的 A, B, C, D 具有相同的长度 N，且 0 ≤ N ≤ 500 。所有整数的范围在 -2<sup>28</sup> 到 2<sup>28</sup> - 1 之间，最终结果不会超过 2<sup>31</sup> - 1 。

#### 例如:
<pre>
<strong>输入:</strong>
A = [ 1, 2]
B = [-2,-1]
C = [-1, 2]
D = [ 0, 2]

<strong>输出:</strong> 
2

<strong>解释:</strong> 
两个元组如下:
1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
</pre>

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for num_a in &a {
            for num_b in &b {
                *map.entry(num_a + num_b).or_insert(0) += 1;
            }
        }
        for num_c in &c {
            for num_d in &d {
                ans += map.get(&-(num_c + num_d)).unwrap_or(&0);
            }
        }
        ans
    }
}
```
