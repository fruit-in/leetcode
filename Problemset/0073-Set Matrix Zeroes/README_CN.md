# 73. 矩阵置零
给定一个 *m* x *n* 的矩阵，如果一个元素为 0，则将其所在行和列的所有元素都设为 0。请使用[**原地**](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)算法。

#### 示例 1:
<pre>
<strong>输入:</strong>
[
  [1,1,1],
  [1,0,1],
  [1,1,1]
]
<strong>输出:</strong>
[
  [1,0,1],
  [0,0,0],
  [1,0,1]
]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
[
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
]
<strong>输出:</strong>
[
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]
</pre>

#### 进阶:
* 一个直接的解决方案是使用  O(*mn*) 的额外空间，但这并不是一个好的解决方案。
* 一个简单的改进方案是使用 O(*m* + *n*) 的额外空间，但这仍然不是最好的解决方案。
* 你能想出一个常数空间的解决方案吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row0 = matrix[0].contains(&0);
        let col0 = matrix.iter().map(|v| v[0]).any(|x| x == 0);
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if col0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if row0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}
```
