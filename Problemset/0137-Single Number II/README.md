# 137. Single Number II
Given an integer array `nums` where every element appears **three times** except for one, which appears **exactly once**. *Find the single element and return it*.

You must implement a solution with a linear runtime complexity and use only constant extra space.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,2,3,2]
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1,0,1,0,1,99]
<strong>Output:</strong> 99
</pre>

#### Constraints:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* Each element in `nums` appears exactly **three times** except for one element which appears **once**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bitcount = [0; 32];
        let mut ret = 0;

        for num in &nums {
            for i in 0..32 {
                if (num >> i) & 1 == 1 {
                    bitcount[i] += 1;
                }
            }
        }

        for i in 0..32 {
            if bitcount[i] % 3 == 1 {
                ret |= 1 << i;
            }
        }

        ret
    }
}
```
