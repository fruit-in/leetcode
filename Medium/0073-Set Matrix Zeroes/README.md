# 73. Set Matrix Zeroes
Given a *m* x *n* matrix, if an element is 0, set its entire row and column to 0. Do it [**in-place**](https://en.wikipedia.org/wiki/In-place_algorithm).

#### Example 1:
<pre>
<strong>Input:</strong>
[
  [1,1,1],
  [1,0,1],
  [1,1,1]
]
<strong>Output:</strong>
[
  [1,0,1],
  [0,0,0],
  [1,0,1]
]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
[
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
]
<strong>Output:</strong>
[
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]
</pre>

#### Follow up:
* A straight forward solution using O(*mn*) space is probably a bad idea.
* A simple improvement uses O(*m* + *n*) space, but still not the best solution.
* Could you devise a constant space solution?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row0 = matrix[0].contains(&0);
        let col0 = matrix.iter().map(|v| v[0]).any(|x| x == 0);
        let m = matrix.len();
        let n = matrix[0].len();

        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            if matrix[i][0] == 0 {
                for j in 1..n {
                    matrix[i][j] = 0;
                }
            }
        }
        for j in 1..n {
            if matrix[0][j] == 0 {
                for i in 1..m {
                    matrix[i][j] = 0;
                }
            }
        }

        if col0 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
        if row0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }
    }
}
```
