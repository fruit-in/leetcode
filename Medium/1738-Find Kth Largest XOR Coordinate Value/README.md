# 1738. Find Kth Largest XOR Coordinate Value
You are given a 2D `matrix` of size `m x n`, consisting of non-negative integers. You are also given an integer `k`.

The **value** of coordinate `(a, b)` of the matrix is the XOR of all `matrix[i][j]` where `0 <= i <= a < m` and `0 <= j <= b < n` (**0-indexed**).

Find the <code>k<sup>th</sup></code> largest value (**1-indexed**) of all the coordinates of `matrix`.

#### Example 1:
<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 1
<strong>Output:</strong> 7
<strong>Explanation:</strong> The value of coordinate (0,1) is 5 XOR 2 = 7, which is the largest value.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The value of coordinate (0,0) is 5 = 5, which is the 2nd largest value.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The value of coordinate (1,0) is 5 XOR 1 = 4, which is the 3rd largest value.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> matrix = [[5,2],[1,6]], k = 4
<strong>Output:</strong> 0
<strong>Explanation:</strong> The value of coordinate (1,1) is 5 XOR 2 XOR 1 XOR 6 = 0, which is the 4th largest value.
</pre>

#### Constraints:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 1000`
* <code>0 <= matrix[i][j] <= 10<sup>6</sup></code>
* `1 <= k <= m * n`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} matrix
# @param {Integer} k
# @return {Integer}
def kth_largest_value(matrix, k)
  vals = []

  (0...matrix.size).each do |i|
    (0...matrix[0].size).each do |j|
      matrix[i][j] ^= matrix[i - 1][j] if i > 0
      matrix[i][j] ^= matrix[i][j - 1] if j > 0
      matrix[i][j] ^= matrix[i - 1][j - 1] if i > 0 && j > 0
      vals.push(matrix[i][j])
    end
  end

  vals.sort[-k]
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn kth_largest_value(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut vals = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 {
                    matrix[i][j] ^= matrix[i - 1][j];
                }
                if j > 0 {
                    matrix[i][j] ^= matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    matrix[i][j] ^= matrix[i - 1][j - 1];
                }
                vals.push(matrix[i][j]);
            }
        }

        vals.sort_unstable_by(|a, b| b.cmp(a));

        vals[k as usize - 1]
    }
}
```
