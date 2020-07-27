# 1464. Maximum Product of Two Elements in an Array
Given the array of integers `nums`, you will choose two different indices `i` and `j` of that array. *Return the maximum value of* `(nums[i]-1)*(nums[j]-1)`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,4,5,2]
<strong>Output:</strong> 12
<strong>Explanation:</strong> If you choose the indices i=1 and j=2 (indexed from 0), you will get the maximum value, that is, (nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5,4,5]
<strong>Output:</strong> 16
<strong>Explanation:</strong> Choosing the indices i=1 and j=3 (indexed from 0), you will get the maximum value of (5-1)*(5-1) = 16.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,7]
<strong>Output:</strong> 12
</pre>

#### Constraints:
* `2 <= nums.length <= 500`
* `1 <= nums[i] <= 10^3`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut i = 1;
        let mut j = 0;

        for k in 2..nums.len() {
            if nums[k] > nums[i] {
                i = k;
            }
        }
        for k in 1..nums.len() {
            if nums[k] > nums[j] && k != i {
                j = k;
            }
        }

        (nums[i] - 1) * (nums[j] - 1)
    }
}
```
