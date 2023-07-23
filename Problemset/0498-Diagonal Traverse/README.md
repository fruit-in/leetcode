# 498. Diagonal Traverse
Given a matrix of M x N elements (M rows, N columns), return all elements of the matrix in diagonal order as shown in the below image.

#### Example:
<pre>
<strong>Input:</strong>
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
<strong>Output:</strong> [1,2,4,7,5,3,6,8,9]
<strong>Explanation:</strong>
<img src="https://assets.leetcode.com/uploads/2018/10/12/diagonal_traverse.png">
</pre>

#### Note:
The total number of elements of the given matrix will not exceed 10,000.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut up_right = true;
        let mut ret = Vec::new();

        for i in 0..(m + n - 1) {
            if up_right {
                let x = i.min(m - 1);
                let y = i - x;
                for j in 0..=(x.min(n - 1 - y)) {
                    ret.push(matrix[x - j][y + j]);
                }
            } else {
                let y = i.min(n - 1);
                let x = i - y;
                for j in 0..=(y.min(m - 1 - x)) {
                    ret.push(matrix[x + j][y - j]);
                }
            }

            up_right = !up_right;
        }

        ret
    }
}
```
