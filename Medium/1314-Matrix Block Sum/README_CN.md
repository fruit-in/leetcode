# 1314. 矩阵区域和
给你一个 ```m * n``` 的矩阵 ```mat``` 和一个整数 ```K``` ，请你返回一个矩阵 ```answer``` ，其中每个 ```answer[i][j]``` 是所有满足下述条件的元素 ```mat[r][c]``` 的和：
* ```i - K <= r <= i + K, j - K <= c <= j + K```
* ```(r, c)``` 在矩阵内。

#### 示例 1:
<pre>
<strong>输入:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], K = 1
<strong>输出:</strong> [[12,21,16],[27,45,33],[24,39,28]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], K = 2
<strong>输出:</strong> [[45,45,45],[45,45,45],[45,45,45]]
</pre>

#### 提示:
* ```m == mat.length```
* ```n == mat[i].length```
* ```1 <= m, n, K <= 100```
* ```1 <= mat[i][j] <= 100```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let k = k as usize;
        let mut row_dp = vec![vec![0; n]; m];
        let mut answer = vec![vec![0; n]; m];

        for i in 0..m {
            row_dp[i][0] = mat[i][..=(k.min(n - 1))].iter().sum();
            for j in 1..n {
                row_dp[i][j] = row_dp[i][j - 1];
                if j - 1 >= k {
                    row_dp[i][j] -= mat[i][j - 1 - k];
                }
                if j + k < n {
                    row_dp[i][j] += mat[i][j + k];
                }
            }
        }

        for j in 0..n {
            answer[0][j] = row_dp[..=(k.min(m - 1))].iter().map(|v| v[j]).sum();
            for i in 1..m {
                answer[i][j] = answer[i - 1][j];
                if i - 1 >= k {
                    answer[i][j] -= row_dp[i - 1 - k][j];
                }
                if i + k < m {
                    answer[i][j] += row_dp[i + k][j];
                }
            }
        }

        answer
    }
}
```
