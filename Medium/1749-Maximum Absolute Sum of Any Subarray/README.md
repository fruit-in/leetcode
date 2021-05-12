# 1749. Maximum Absolute Sum of Any Subarray
You are given an integer array `nums`. The **absolute sum** of a subarray <code>[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]</code> is <code>abs(nums<sub>l</sub> + nums<sub>l+1</sub> + ... + nums<sub>r-1</sub> + nums<sub>r</sub>)</code>.

Return *the **maximum** absolute sum of any **(possibly empty)** subarray of* `nums`.

Note that `abs(x)` is defined as follows:
* If `x` is a negative integer, then `abs(x) = -x`.
* If `x` is a non-negative integer, then `abs(x) = x`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,-3,2,3,-4]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The subarray [2,3] has absolute sum = abs(2+3) = abs(5) = 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,-5,1,-4,3,-2]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The subarray [-5,1,-4] has absolute sum = abs(-5+1-4) = abs(-8) = 8.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Prefix Sum
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def max_absolute_sum(nums)
  sum = 0
  max_sum = 0
  min_sum = 0
  ret = 0

  nums.each do |x|
    sum += x
    max_sum = [max_sum, sum].max
    min_sum = [min_sum, sum].min
    ret = [ret, (sum - max_sum).abs, (sum - min_sum).abs].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Prefix Sum
```Rust
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut ret = 0;

        for x in nums {
            sum += x;
            max_sum = max_sum.max(sum);
            min_sum = min_sum.min(sum);
            ret = ret.max((sum - max_sum).abs()).max((sum - min_sum).abs());
        }

        ret
    }
}
```
