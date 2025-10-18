# 2640. Find the Score of All Prefixes of an Array
We define the **conversion array** `conver` of an array `arr` as follows:
* `conver[i] = arr[i] + max(arr[0..i])` where `max(arr[0..i])` is the maximum value of `arr[j]` over `0 <= j <= i`.

We also define the **score** of an array `arr` as the sum of the values of the conversion array of `arr`.

Given a **0-indexed** integer array `nums` of length `n`, return *an array* `ans` *of length* `n` *where* `ans[i]` *is the score of the prefix* `nums[0..i]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,7,5,10]
<strong>Output:</strong> [4,10,24,36,56]
<strong>Explanation:</strong>
For the prefix [2], the conversion array is [4] hence the score is 4
For the prefix [2, 3], the conversion array is [4, 6] hence the score is 10
For the prefix [2, 3, 7], the conversion array is [4, 6, 14] hence the score is 24
For the prefix [2, 3, 7, 5], the conversion array is [4, 6, 14, 12] hence the score is 36
For the prefix [2, 3, 7, 5, 10], the conversion array is [4, 6, 14, 12, 20] hence the score is 56
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,2,4,8,16]
<strong>Output:</strong> [2,4,8,16,32,64]
<strong>Explanation:</strong>
For the prefix [1], the conversion array is [2] hence the score is 2
For the prefix [1, 1], the conversion array is [2, 2] hence the score is 4
For the prefix [1, 1, 2], the conversion array is [2, 2, 4] hence the score is 8
For the prefix [1, 1, 2, 4], the conversion array is [2, 2, 4, 8] hence the score is 16
For the prefix [1, 1, 2, 4, 8], the conversion array is [2, 2, 4, 8, 16] hence the score is 32
For the prefix [1, 1, 2, 4, 8, 16], the conversion array is [2, 2, 4, 8, 16, 32] hence the score is 64
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut max_num = 0;
        let mut conver = vec![0; n];
        let mut ans = vec![0; n];

        for i in 0..n {
            max_num = max_num.max(nums[i]);
            conver[i] = nums[i] + max_num;
            ans[i] = ans[i.max(1) - 1] + conver[i] as i64;
        }

        ans
    }
}
```
