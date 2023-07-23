# 542. 01 矩阵
给定一个由 0 和 1 组成的矩阵，找出每个元素到最近的 0 的距离。

两个相邻元素间的距离为 1 。

#### 示例 1:
输入:
```
0 0 0
0 1 0
0 0 0
```
输出:
```
0 0 0
0 1 0
0 0 0
```

#### 示例 2:
输入:
```
0 0 0
0 1 0
1 1 1
```
输出:
```
0 0 0
0 1 0
1 2 1
```

#### 注意:
1. 给定矩阵的元素个数不超过 10000。
2. 给定矩阵中至少有一个元素是 0。
3. 矩阵中的元素只在四个方向上相邻: 上、下、左、右。

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![10000; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                match matrix[i][j] {
                    0 => ret[i][j] = 0,
                    _ => {
                        if i > 0 {
                            ret[i][j] = ret[i - 1][j] + 1;
                        }
                        if j > 0 {
                            ret[i][j] = ret[i][j].min(ret[i][j - 1] + 1);
                        }
                    }
                }
            }
        }

        for i in (0..matrix.len()).rev() {
            for j in (0..matrix[0].len()).rev() {
                if matrix[i][j] == 1 {
                    if i + 1 < matrix.len() {
                        ret[i][j] = ret[i][j].min(ret[i + 1][j] + 1);
                    }
                    if j + 1 < matrix[0].len() {
                        ret[i][j] = ret[i][j].min(ret[i][j + 1] + 1);
                    }
                }
            }
        }

        ret
    }
}
```
