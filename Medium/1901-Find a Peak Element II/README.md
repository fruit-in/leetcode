# 1901. Find a Peak Element II
A **peak** element in a 2D grid is an element that is **strictly greater** than all of its **adjacent** neighbors to the left, right, top, and bottom.

Given a **0-indexed** `m x n` matrix `mat` where **no two adjacent cells are equal**, find **any** peak element `mat[i][j]` and return *the length 2 array* `[i,j]`.

You may assume that the entire matrix is surrounded by an **outer perimeter** with the value `-1` in each cell.

You must write an algorithm that runs in `O(m log(n))` or `O(n log(m))` time.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/08/1.png)
<pre>
<strong>Input:</strong> mat = [[1,4],[3,2]]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> Both 3 and 4 are peak elements so [1,0] and [0,1] are both acceptable answers.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/07/3.png)
<pre>
<strong>Input:</strong> mat = [[10,20,15],[21,30,14],[7,16,32]]
<strong>Output:</strong> [1,1]
<strong>Explanation:</strong> Both 30 and 32 are peak elements so [1,1] and [2,2] are both acceptable answers.
</pre>

#### Constraints:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 500`
* <code>1 <= mat[i][j] <= 10<sup>5</sup></code>
* No two adjacent cells are equal.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut i = 0;
        let mut j = 0;

        loop {
            let mut peak_i = i;
            let mut peak_j = j;

            if i > 0 && mat[i - 1][j] > mat[peak_i][peak_j] {
                peak_i = i - 1;
                peak_j = j;
            }
            if i < mat.len() - 1 && mat[i + 1][j] > mat[peak_i][peak_j] {
                peak_i = i + 1;
                peak_j = j;
            }
            if j > 0 && mat[i][j - 1] > mat[peak_i][peak_j] {
                peak_i = i;
                peak_j = j - 1;
            }
            if j < mat[0].len() - 1 && mat[i][j + 1] > mat[peak_i][peak_j] {
                peak_i = i;
                peak_j = j + 1;
            }

            if peak_i == i && peak_j == j {
                break;
            }

            i = peak_i;
            j = peak_j;
        }

        vec![i as i32, j as i32]
    }
}
```
