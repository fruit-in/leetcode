# 945. Minimum Increment to Make Array Unique
Given an array of integers nums, a *move* consists of choosing any `nums[i]`, and incrementing it by `1`.

Return the least number of moves to make every value in `nums` unique.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> After 1 move, the array could be [1, 2, 3].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,2,1,2,1,7]
<strong>Output:</strong> 6
<strong>Explanation:</strong> After 6 moves, the array could be [3, 4, 1, 2, 5, 7].
It can be shown with 5 or less moves that it is impossible for the array to have all unique values.
</pre>

#### Note:
1. `0 <= nums.length <= 40000`
2. `0 <= nums[i] < 40000`

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_increment_for_unique(nums)
  nums.sort!
  ret = 0

  (1...nums.size).each do |i|
    ret += [nums[i], nums[i - 1] + 1].max - nums[i]
    nums[i] = [nums[i], nums[i - 1] + 1].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += nums[i].max(nums[i - 1] + 1) - nums[i];
            nums[i] = nums[i].max(nums[i - 1] + 1);
        }

        ret
    }
}
```
