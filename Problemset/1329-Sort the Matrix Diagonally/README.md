# 1329. Sort the Matrix Diagonally
Given a `m * n` matrix `mat` of integers, sort it diagonally in ascending order from the top-left to the bottom-right then return the sorted array.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/21/1482_example_1_2.png)
<pre>
<strong>Input:</strong> mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
<strong>Output:</strong> [[1,1,1,1],[1,2,2,2],[1,2,3,3]]
</pre>

#### Constraints:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 100`
* `1 <= mat[i][j] <= 100`

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
