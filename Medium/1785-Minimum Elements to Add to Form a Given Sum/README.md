# 1785. Minimum Elements to Add to Form a Given Sum
You are given an integer array `nums` and two integers `limit` and `goal`. The array `nums` has an interesting property that `abs(nums[i]) <= limit`.

Return *the minimum number of elements you need to add to make the sum of the array equal to* `goal`. The array must maintain its property that `abs(nums[i]) <= limit`.

Note that `abs(x)` equals `x` if `x >= 0`, and `-x` otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,-1,1], limit = 3, goal = -4
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can add -2 and -3, then the sum of the array will be 1 - 1 + 1 - 2 - 3 = -4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,-10,9,1], limit = 100, goal = 0
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= limit <= 10<sup>6</sup></code>
* `-limit <= nums[i] <= limit`
* <code>-10<sup>9</sup> <= goal <= 10<sup>9</sup></code>

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer[]} nums
# @param {Integer} limit
# @param {Integer} goal
# @return {Integer}
def min_elements(nums, limit, goal)
  ((nums.sum - goal).abs * 1.0 / limit).ceil
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        ((nums.iter().map(|&x| x as i64).sum::<i64>() - goal as i64).abs() as f64 / limit as f64)
            .ceil() as i32
    }
}
```
