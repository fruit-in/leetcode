# 59. Spiral Matrix II
Given a positive integer *n*, generate a square matrix filled with elements from 1 to *n*<sup>2</sup> in spiral order.

#### Example:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong>
[
 [ 1, 2, 3 ],
 [ 8, 9, 4 ],
 [ 7, 6, 5 ]
]
</pre>

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut ret = vec![vec![0; n]; n];

        let mut num = 1;
        for i in 0..((n + 1) / 2) {
            for c in i..(n - 1 - i) {
                ret[i][c] = num;
                num += 1;
            }
            for r in i..(n - 1 - i) {
                ret[r][n - 1 - i] = num;
                num += 1;
            }
            for c in ((i + 1)..(n - i)).rev() {
                ret[n - 1 - i][c] = num;
                num += 1;
            }
            for r in ((i + 1)..(n - i)).rev() {
                ret[r][i] = num;
                num += 1;
            }
        }
        ret[n / 2][(n - 1) / 2] = (n * n) as i32;

        ret
    }
}
```
