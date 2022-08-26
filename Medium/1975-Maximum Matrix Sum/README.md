# 1975. Maximum Matrix Sum
You are given an `n x n` integer `matrix`. You can do the following operation **any** number of times:
* Choose any two **adjacent** elements of `matrix` and **multiply** each of them by `-1`.

Two elements are considered **adjacent** if and only if they share a **border**.

Your goal is to **maximize** the summation of the matrix's elements. Return *the **maximum** sum of the matrix's elements using the operation mentioned above*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex1.png)
<pre>
<strong>Input:</strong> matrix = [[1,-1],[-1,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can follow the following steps to reach sum equals 4:
- Multiply the 2 elements in the first row by -1.
- Multiply the 2 elements in the first column by -1.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex2.png)
<pre>
<strong>Input:</strong> matrix = [[1,2,3],[-1,-2,-3],[1,2,3]]
<strong>Output:</strong> 16
<strong>Explanation:</strong> We can follow the following step to reach sum equals 16:
- Multiply the 2 last elements in the second row by -1.
</pre>

#### Constraints:
* `n == matrix.length == matrix[i].length`
* `2 <= n <= 250`
* <code>-10<sup>5</sup> <= matrix[i][j] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut count_neg_even = true;
        let mut min_abs = i32::MAX;
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j] < 0 {
                    count_neg_even = !count_neg_even;
                }
                min_abs = min_abs.min(matrix[i][j].abs());
                ret += matrix[i][j].abs() as i64;
            }
        }

        if !count_neg_even {
            ret -= 2 * min_abs as i64;
        }

        ret
    }
}
```
