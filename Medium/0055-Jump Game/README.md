# 55. Jump Game
Given an array of non-negative integers `nums`, you are initially positioned at the **first index** of the array.

Each element in the array represents your maximum jump length at that position.

Determine if you are able to reach the last index.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,1,1,4]
<strong>Output:</strong> true
<strong>Explanation:</strong> Jump 1 step from index 0 to 1, then 3 steps to the last index.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,2,1,0,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def can_jump(nums)
  max_index = 0

  (0...nums.size).each do |i|
    return false if i > max_index

    max_index = [max_index, i + nums[i]].max
  end

  true
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_index = 0;

        for i in 0..nums.len() {
            if i > max_index as usize {
                return false;
            }
            max_index = max_index.max(i as i32 + nums[i]);
        }

        true
    }
}
```
