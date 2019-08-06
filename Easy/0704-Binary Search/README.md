# 704. Binary Search
Given a **sorted** (in ascending order) integer array <code>nums</code> of <code>n</code> elements and a <code>target</code> value, write a function to search <code>target</code> in <code>nums</code>. If <code>target</code> exists, then return its index, otherwise return <code>-1</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-1,0,3,5,9,12], target = 9
<strong>Output:</strong> 4
<strong>Explanation:</strong> 9 exists in nums and its index is 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1,0,3,5,9,12], target = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> 2 does not exist in nums so return -1
</pre>

#### Note:
1. You may assume that all elements in <code>nums</code> are unique.
2. <code>n</code> will be in the range <code>[1, 10000]</code>.
3. The value of each element in <code>nums</code> will be in the range <code>[-9999, 9999]</code>.

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut head = 0;
        let mut tail = nums.len() as i32 - 1;
        while head <= tail {
            if target == nums[(head + tail) as usize / 2] {
                return (head + tail) / 2;
            } else if target > nums[(head + tail) as usize / 2] {
                head = (head + tail) / 2 + 1;
            } else if target < nums[(head + tail) as usize / 2] {
                tail = (head + tail) / 2 - 1;
            }
        }
        -1
    }
}
```
