# 59. 螺旋矩阵 II
给定一个正整数 *n*，生成一个包含 1 到 *n*<sup>2</sup> 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。

#### 示例:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong>
[
 [ 1, 2, 3 ],
 [ 8, 9, 4 ],
 [ 7, 6, 5 ]
]
</pre>

## 题解 (Rust)

### 1. 模拟
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
