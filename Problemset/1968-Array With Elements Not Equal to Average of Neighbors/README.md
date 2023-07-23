# 1968. Array With Elements Not Equal to Average of Neighbors
You are given a **0-indexed** array `nums` of **distinct** integers. You want to rearrange the elements in the array such that every element in the rearranged array is **not** equal to the **average** of its neighbors.

More formally, the rearranged array should have the property such that for every `i` in the range `1 <= i < nums.length - 1`, `(nums[i-1] + nums[i+1]) / 2` is **not** equal to `nums[i]`.

Return ***any** rearrangement of* `nums` *that meets the requirements*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> [1,2,4,5,3]
<strong>Explanation:</strong>
When i=1, nums[i] = 2, and the average of its neighbors is (1+4) / 2 = 2.5.
When i=2, nums[i] = 4, and the average of its neighbors is (2+5) / 2 = 3.5.
When i=3, nums[i] = 5, and the average of its neighbors is (4+3) / 2 = 3.5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [6,2,0,9,7]
<strong>Output:</strong> [9,7,6,2,0]
<strong>Explanation:</strong>
When i=1, nums[i] = 7, and the average of its neighbors is (9+6) / 2 = 7.5.
When i=2, nums[i] = 6, and the average of its neighbors is (7+2) / 2 = 4.5.
When i=3, nums[i] = 2, and the average of its neighbors is (6+0) / 2 = 3.
</pre>

#### Constraints:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = vec![];
        nums.sort_unstable();

        if nums.len() % 2 == 1 {
            ret.push(nums.pop().unwrap());
        }

        for i in 0..nums.len() / 2 {
            ret.push(nums[i]);
            ret.push(nums[nums.len() / 2 + i]);
        }

        ret
    }
}
```
