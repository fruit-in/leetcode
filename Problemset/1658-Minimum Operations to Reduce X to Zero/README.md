# 1658. Minimum Operations to Reduce X to Zero
You are given an integer array `nums` and an integer `x`. In one operation, you can either remove the leftmost or the rightmost element from the array `nums` and subtract its value from `x`. Note that this **modifies** the array for future operations.

Return *the **minimum number** of operations to reduce* `x` *to **exactly*** `0` *if it's possible, otherwise, return* `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,4,2,3], x = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> The optimal solution is to remove the last two elements to reduce x to zero.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,6,7,8,9], x = 4
<strong>Output:</strong> -1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [3,2,20,1,1,3], x = 10
<strong>Output:</strong> 5
<strong>Explanation:</strong> The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* <code>1 <= x <= 10<sup>9</sup></code>

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {Integer[]} nums
# @param {Integer} x
# @return {Integer}
def min_operations(nums, x)
  l = nums.size
  r = 0
  sum = nums.sum
  ret = -1

  while r < nums.size
    ret = l + r if sum == x && (ret == -1 || l + r < ret)
    if (sum > x && l > 0) || l + r >= nums.size
      l -= 1
      sum -= nums[l]
    else
      r += 1
      sum += nums[-r]
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut l = nums.len();
        let mut r = 0;
        let mut sum = nums.iter().sum::<i32>();
        let mut ret = -1;

        while r < nums.len() {
            if sum == x && (ret == -1 || l + r < ret as usize) {
                ret = (l + r) as i32;
            }
            if (sum > x && l > 0) || l + r >= nums.len() {
                l -= 1;
                sum -= nums[l];
            } else {
                r += 1;
                sum += nums[nums.len() - r];
            }
        }

        ret
    }
}
```
