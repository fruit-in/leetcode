# 1186. Maximum Subarray Sum with One Deletion
Given an array of integers, return the maximum sum for a **non-empty** subarray (contiguous elements) with at most one element deletion. In other words, you want to choose a subarray and optionally delete one element from it so that there is still at least one element left and the sum of the remaining elements is maximum possible.

Note that the subarray needs to be **non-empty** after deleting one element.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,-2,0,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Because we can choose [1, -2, 0, 3] and drop -2, thus the subarray [1, 0, 3] becomes the maximum value.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,-2,-2,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We just choose [3] and it's the maximum sum.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [-1,-1,-1,-1]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The final subarray needs to be non-empty. You can't choose [-1] and delete -1 from it, then get an empty subarray to make the sum equals to 0.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= arr[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let (mut x, mut y) = (arr[0], 0);
        let mut ret = x;

        for i in 1..arr.len() {
            (x, y) = (arr[i].max(x + arr[i]), x.max(y + arr[i]));
            ret = ret.max(x).max(y);
        }

        ret
    }
}
```
