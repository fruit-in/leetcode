# 54. Spiral Matrix
Given a matrix of *m* x *n* elements (*m* rows, *n* columns), return all elements of the matrix in spiral order.

#### Example 1:
<pre>
<strong>Input:</strong>
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
<strong>Output:</strong> [1,2,3,6,9,8,7,4,5]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
[
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9,10,11,12]
]
<strong>Output:</strong> [1,2,3,4,8,12,11,10,9,5,6,7]
</pre>

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return Vec::new();
        }

        let mut row = 0;
        let mut col = 0;
        let mut u = 0;
        let mut d = matrix.len() as i32 - 1;
        let mut l = 0;
        let mut r = matrix[0].len() as i32 - 1;
        let mut dir = (0, 1);
        let mut ret = Vec::new();

        for _ in 0..(matrix.len() * matrix[0].len()) {
            ret.push(matrix[row as usize][col as usize]);

            match dir {
                (0, 1) if col == r => { dir = (1, 0); u += 1; },
                (1, 0) if row == d => { dir = (0, -1); r -= 1; },
                (0, -1) if col == l => { dir = (-1, 0); d -= 1; },
                (-1, 0) if row == u => { dir = (0, 1); l += 1; },
                _ => (),
            }

            row += dir.0;
            col += dir.1;
        }

        ret
    }
}
```
