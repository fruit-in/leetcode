# 1380. Lucky Numbers in a Matrix
Given a ```m * n``` matrix of **distinct** numbers, return all lucky numbers in the matrix in **any** order.

A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

#### Example 1:
<pre>
<strong>Input:</strong> matrix = [[3,7,8],[9,11,13],[15,16,17]]
<strong>Output:</strong> [15]
<strong>Explanation:</strong> 15 is the only lucky number since it is the minimum in its row and the maximum in its column
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
<strong>Output:</strong> [12]
<strong>Explanation:</strong> 12 is the only lucky number since it is the minimum in its row and the maximum in its column.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [[7,8],[1,2]]
<strong>Output:</strong> [7]
</pre>

#### Constraints:
* ```m == mat.length```
* ```n == mat[i].length```
* ```1 <= n, m <= 50```
* ```1 <= matrix[i][j] <= 10^5```.
* All elements in the matrix are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_min = vec![100001; m];
        let mut col_max = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                row_min[i] = row_min[i].min(matrix[i][j]);
                col_max[j] = col_max[j].max(matrix[i][j]);
            }
        }

        row_min.drain(..).filter(|x| col_max.contains(x)).collect()
    }
}
```
