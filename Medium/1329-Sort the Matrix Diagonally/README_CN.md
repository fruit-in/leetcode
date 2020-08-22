# 1329. 将矩阵按对角线排序
给你一个 `m * n` 的整数矩阵 `mat` ，请你将同一条对角线上的元素（从左上到右下）按升序排序后，返回排好序的矩阵。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/25/1482_example_1_2.png)
<pre>
<strong>输入:</strong> mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
<strong>输出:</strong> [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
</pre>

#### 提示:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 100`
* `1 <= mat[i][j] <= 100`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} mat
# @return {Integer[][]}
def diagonal_sort(mat)
    m, n = mat.length, mat[0].length

    for i in 0...(m + n)
        row = [m - 1 - i, 0].max
        col = [i - m + 1, 0].max
        arr = []

        for j in 0...[m - row, n - col].min
            arr.push(mat[row + j][col + j])
        end

        arr.sort!

        for j in 0...[m - row, n - col].min
            mat[row + j][col + j] = arr[j]
        end
    end

    return mat
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..(m + n) {
            let mut row = (m - 1).saturating_sub(i);
            let mut col = i.saturating_sub(m - 1);
            let mut arr = vec![];

            for j in 0..(m - row).min(n - col) {
                arr.push(mat[row + j][col + j]);
            }

            arr.sort_unstable();

            for j in 0..(m - row).min(n - col) {
                ret[row + j][col + j] = arr[j];
            }
        }

        ret
    }
}
```
