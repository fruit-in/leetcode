# 566. Reshape the Matrix
In MATLAB, there is a very useful function called 'reshape', which can reshape a matrix into a new one with different size but keep its original data.

You're given a matrix represented by a two-dimensional array, and two **positive** integers **r** and **c** representing the **row** number and **column** number of the wanted reshaped matrix, respectively.

The reshaped matrix need to be filled with all the elements of the original matrix in the same **row-traversing** order as they were.

If the 'reshape' operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.

#### Example 1:
<pre>
<strong>Input:</strong>
nums =
[[1,2],
 [3,4]]
r = 1, c = 4
<strong>Output:</strong>
[[1,2,3,4]]
<strong>Explanation:</strong>
The <strong>row-traversing</strong> of nums is [1,2,3,4]. The new reshaped matrix is a 1 * 4 matrix, fill it row by row by using the previous list.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
nums =
[[1,2],
 [3,4]]
r = 2, c = 4
<strong>Output:</strong>
[[1,2],
 [3,4]]
<strong>Explanation:</strong>
There is no way to reshape a 2 * 2 matrix to a 2 * 4 matrix. So output the original matrix.
</pre>

#### Note:
1. The height and width of the given matrix is in range [1, 100].
2. The given r and c are all positive.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if nums.len() * nums[0].len() != (r * c) as usize {
            return nums;
        }

        nums.concat()
            .chunks(c as usize)
            .map(|row| row.to_vec())
            .collect()
    }
}
```
