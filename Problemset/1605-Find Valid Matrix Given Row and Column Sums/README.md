# 1605. Find Valid Matrix Given Row and Column Sums
You are given two arrays `rowSum` and `colSum` of non-negative integers where `rowSum[i]` is the sum of the elements in the <code>i<sup>th</sup></code> row and `colSum[j]` is the sum of the elements of the <code>j<sup>th</sup></code> column of a 2D matrix. In other words, you do not know the elements of the matrix, but you do know the sums of each row and column.

Find any matrix of **non-negative** integers of size `rowSum.length x colSum.length` that satisfies the `rowSum` and `colSum` requirements.

Return *a 2D array representing **any** matrix that fulfills the requirements*. It's guaranteed that **at least one** matrix that fulfills the requirements exists.

#### Example 1:
<pre>
<strong>Input:</strong> rowSum = [3,8], colSum = [4,7]
<strong>Output:</strong> [[3,0],
         [1,7]]
<strong>Explanation:</strong>
0th row: 3 + 0 = 3 == rowSum[0]
1st row: 1 + 7 = 8 == rowSum[1]
0th column: 3 + 1 = 4 == colSum[0]
1st column: 0 + 7 = 7 == colSum[1]
The row and column sums match, and all matrix elements are non-negative.
Another possible matrix is: [[1,2],
                             [3,5]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rowSum = [5,7,10], colSum = [8,6,8]
<strong>Output:</strong> [[0,5,0],
         [6,1,0],
         [2,0,8]]
</pre>

#### Constraints:
* `1 <= rowSum.length, colSum.length <= 500`
* <code>0 <= rowSum[i], colSum[i] <= 10<sup>8</sup></code>
* `sum(rowSum) == sum(colSum)`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row = 0;
        let mut col = 0;
        let mut ret = vec![vec![0; col_sum.len()]; row_sum.len()];

        while row < row_sum.len() && col < col_sum.len() {
            if row_sum[row] < col_sum[col] {
                ret[row][col] = row_sum[row];
                col_sum[col] -= row_sum[row];
                row += 1;
            } else if row_sum[row] > col_sum[col] {
                ret[row][col] = col_sum[col];
                row_sum[row] -= col_sum[col];
                col += 1;
            } else {
                ret[row][col] = row_sum[row];
                row += 1;
                col += 1;
            }
        }

        ret
    }
}
```
