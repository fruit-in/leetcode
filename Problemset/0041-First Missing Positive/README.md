# 41. First Missing Positive
Given an unsorted integer array `nums`, return the smallest missing positive integer.

You must implement an algorithm that runs in `O(n)` time and uses `O(1)` auxiliary space.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The numbers in the range [1,2] are all in the array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,4,-1,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> 1 is in the array but 2 is missing.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [7,8,9,11,12]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The smallest positive integer 1 is missing.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.push(0);

        for i in 0..nums.len() {
            while nums[i] >= 0 && nums[i] < nums.len() as i32 && nums[nums[i] as usize] != nums[i] {
                let tmp = nums[i] as usize;
                nums.swap(tmp, i);
            }
        }

        for i in 1..nums.len() {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }

        nums.len() as i32
    }
}
```
