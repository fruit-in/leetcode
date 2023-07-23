# 74. Search a 2D Matrix
Write an efficient algorithm that searches for a value in an *m* x *n* matrix. This matrix has the following properties:
* Integers in each row are sorted from left to right.
* The first integer of each row is greater than the last integer of the previous row.

#### Example 1:
<pre>
<strong>Input:</strong>
matrix = [
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
]
target = 3
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
matrix = [
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
]
target = 13
<strong>Output:</strong> false
</pre>

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut l = 0;
        let mut r = m * n;

        while l < r {
            let mid = (l + r) / 2;
            let row = mid / n;
            let col = mid % n;

            if target == matrix[row][col] {
                return true;
            } else if target < matrix[row][col] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        false
    }
}
```
