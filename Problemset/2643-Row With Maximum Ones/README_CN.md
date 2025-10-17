# 2643. 一最多的行
给你一个大小为 `m x n` 的二进制矩阵 `mat` ，请你找出包含最多 **1** 的行的下标（从 **0** 开始）以及这一行中 **1** 的数目。

如果有多行包含最多的 1 ，只需要选择 **行下标最小** 的那一行。

返回一个由行下标和该行中 1 的数量组成的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> mat = [[0,1],[1,0]]
<strong>输出:</strong> [0,1]
<strong>解释:</strong> 两行中 1 的数量相同。所以返回下标最小的行，下标为 0 。该行 1 的数量为 1 。所以，答案为 [0,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> mat = [[0,0,0],[0,1,1]]
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 下标为 1 的行中 1 的数量最多。该行 1 的数量为 2 。所以，答案为 [1,2] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> mat = [[0,0],[1,1],[0,0]]
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 下标为 1 的行中 1 的数量最多。该行 1 的数量为 2 。所以，答案为 [1,2] 。
</pre>

#### 提示:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 100`
* `mat[i][j]` 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0, 0];

        for i in 0..mat.len() {
            let ones = mat[i].iter().sum::<i32>();

            if ones > ret[1] {
                ret = vec![i as i32, ones];
            }
        }

        ret
    }
}
```
