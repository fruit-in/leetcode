# 1144. Decrease Elements To Make Array Zigzag
Given an array `nums` of integers, a *move* consists of choosing any element and **decreasing it by 1**.

An array `A` is a *zigzag array* if either:
* Every even-indexed element is greater than adjacent elements, ie. `A[0] > A[1] < A[2] > A[3] < A[4] > ...`
* OR, every odd-indexed element is greater than adjacent elements, ie. `A[0] < A[1] > A[2] < A[3] > A[4] < ...`

Return the minimum number of moves to transform the given array `nums` into a zigzag array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can decrease 2 to 0 or 3 to 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [9,6,1,6,2]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        Self::moves_to_make_v(&nums, true).min(Self::moves_to_make_v(&nums, false))
    }

    fn moves_to_make_v(nums: &[i32], odd: bool) -> i32 {
        (odd as usize..nums.len())
            .step_by(2)
            .map(|i| {
                nums[i]
                    - nums[i]
                        .min(*nums.get(i - 1).unwrap_or(&1000) - 1)
                        .min(*nums.get(i + 1).unwrap_or(&1000) - 1)
            })
            .sum()
    }
}
```
