# 454. 4Sum II
Given four lists A, B, C, D of integer values, compute how many tuples <code>(i, j, k, l)</code> there are such that <code>A[i] + B[j] + C[k] + D[l]</code> is zero.

To make problem a bit easier, all A, B, C, D have same length of N where 0 ≤ N ≤ 500. All integers are in the range of -2<sup>28</sup> to 2<sup>28</sup> - 1 and the result is guaranteed to be at most 2<sup>31</sup> - 1.

#### Example:
<pre>
<strong>Input:</strong>
A = [ 1, 2]
B = [-2,-1]
C = [-1, 2]
D = [ 0, 2]

<strong>Output:</strong> 
2

<strong>Explanation:</strong> 
The two tuples are:
1. (0, 0, 0, 1) -> A[0] + B[0] + C[0] + D[1] = 1 + (-2) + (-1) + 2 = 0
2. (1, 1, 0, 0) -> A[1] + B[1] + C[0] + D[0] = 2 + (-1) + (-1) + 0 = 0
</pre>

## Solutions

### 1. Hash Map (Rust)
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
