# 1887. Reduction Operations to Make the Array Elements Equal
Given an integer array `nums`, your goal is to make all elements in `nums` equal. To complete one operation, follow these steps:
1. Find the **largest** value in `nums`. Let its index be `i` (**0-indexed**) and its value be `largest`. If there are multiple elements with the largest value, pick the smallest `i`.
2. Find the **next largest** value in `nums` **strictly smaller** than `largest`. Let its value be `nextLargest`.
3. Reduce `nums[i]` to `nextLargest`.

Return *the number of operations to make all elements in* `nums` *equal*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,1,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> It takes 3 operations to make all elements in nums equal:
1. largest = 5 at index 0. nextLargest = 3. Reduce nums[0] to 3. nums = [3,1,3].
2. largest = 3 at index 0. nextLargest = 1. Reduce nums[0] to 1. nums = [1,1,3].
3. largest = 3 at index 2. nextLargest = 1. Reduce nums[2] to 1. nums = [1,1,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> All elements in nums are already equal.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,2,2,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> It takes 4 operations to make all elements in nums equal:
1. largest = 3 at index 4. nextLargest = 2. Reduce nums[4] to 2. nums = [1,1,2,2,2].
2. largest = 2 at index 2. nextLargest = 1. Reduce nums[2] to 1. nums = [1,1,1,2,2].
3. largest = 2 at index 3. nextLargest = 1. Reduce nums[3] to 1. nums = [1,1,1,1,2].
4. largest = 2 at index 4. nextLargest = 1. Reduce nums[4] to 1. nums = [1,1,1,1,1].
</pre>

#### Constraints:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 5 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0;
        let mut ret = 0;
        nums.sort_unstable();
        let mut prev = nums[0];

        for &num in &nums[1..] {
            if num != prev {
                count += 1;
                prev = num;
            }

            ret += count;
        }

        ret
    }
}
```
