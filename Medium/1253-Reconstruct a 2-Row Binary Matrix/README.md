# 1253. Reconstruct a 2-Row Binary Matrix
Given the following details of a matrix with `n` columns and `2` rows :
* The matrix is a binary matrix, which means each element in the matrix can be `0` or `1`.
* The sum of elements of the 0-th(upper) row is given as `upper`.
* The sum of elements of the 1-st(lower) row is given as `lower`.
* The sum of elements in the i-th column(0-indexed) is `colsum[i]`, where `colsum` is given as an integer array with length `n`.

Your task is to reconstruct the matrix with `upper`, `lower` and `colsum`.

Return it as a 2-D integer array.

If there are more than one valid solution, any of them will be accepted.

If no valid solution exists, return an empty 2-D array.

#### Example 1:
<pre>
<strong>Input:</strong> upper = 2, lower = 1, colsum = [1,1,1]
<strong>Output:</strong> [[1,1,0],[0,0,1]]
<strong>Explanation:</strong> [[1,0,1],[0,1,0]], and [[0,1,1],[1,0,0]] are also correct answers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> upper = 2, lower = 3, colsum = [2,2,1,1]
<strong>Output:</strong> []
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
<strong>Output:</strong> [[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]
</pre>

#### Constraints:
* `1 <= colsum.length <= 10^5`
* `0 <= upper, lower <= colsum.length`
* `0 <= colsum[i] <= 2`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer} upper
# @param {Integer} lower
# @param {Integer[]} colsum
# @return {Integer[][]}
def reconstruct_matrix(upper, lower, colsum)
  ret = Array.new(2) { [0] * colsum.size }

  (0...colsum.size).each do |i|
    if colsum[i] == 2
      ret[0][i] = 1
      ret[1][i] = 1
      upper -= 1
      lower -= 1
    elsif colsum[i] == 1
      if upper > lower
        ret[0][i] = 1
        upper -= 1
      else
        ret[1][i] = 1
        lower -= 1
      end
    end
  end

  upper | lower == 0 ? ret : []
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; colsum.len()]; 2];

        for i in 0..colsum.len() {
            if colsum[i] == 2 {
                ret[0][i] = 1;
                ret[1][i] = 1;
                upper -= 1;
                lower -= 1;
            } else if colsum[i] == 1 {
                if upper > lower {
                    ret[0][i] = 1;
                    upper -= 1;
                } else {
                    ret[1][i] = 1;
                    lower -= 1;
                }
            }
        }

        if upper | lower != 0 {
            vec![]
        } else {
            ret
        }
    }
}
```
