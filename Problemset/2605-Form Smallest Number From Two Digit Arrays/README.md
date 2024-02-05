# 2605. Form Smallest Number From Two Digit Arrays
Given two arrays of **unique** digits `nums1` and `nums2`, return *the **smallest** number that contains **at least** one digit from each array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [4,1,3], nums2 = [5,7]
<strong>Output:</strong> 15
<strong>Explanation:</strong> The number 15 contains the digit 1 from nums1 and the digit 5 from nums2. It can be proven that 15 is the smallest number we can have.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [3,5,2,6], nums2 = [3,1,7]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The number 3 contains the digit 3 which exists in both arrays.
</pre>

#### Constraints:
* `1 <= nums1.length, nums2.length <= 9`
* `1 <= nums1[i], nums2[i] <= 9`
* All digits in each array are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min0 = 10;
        let min1 = *nums1.iter().min().unwrap();
        let min2 = *nums2.iter().min().unwrap();

        for x in &nums1 {
            for y in &nums2 {
                if x == y {
                    min0 = min0.min(*x);
                }
            }
        }

        if min0 < 10 {
            min0
        } else if min1 < min2 {
            min1 * 10 + min2
        } else {
            min2 * 10 + min1
        }
    }
}
```
