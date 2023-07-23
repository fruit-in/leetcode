# 1749. 任意子数组和的绝对值的最大值
给你一个整数数组 `nums` 。一个子数组 <code>[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]</code> 的 **和的绝对值** 为 <code>abs(nums<sub>l</sub> + nums<sub>l+1</sub> + ... + nums<sub>r-1</sub> + nums<sub>r</sub>)</code> 。

请你找出 `nums` 中 **和的绝对值** 最大的任意子数组（**可能为空**），并返回该 **最大值** 。

`abs(x)` 定义如下：
* 如果 `x` 是负整数，那么 `abs(x) = -x` 。
* 如果 `x` 是非负整数，那么 `abs(x) = x` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,-3,2,3,-4]
<strong>输出:</strong> 5
<strong>解释:</strong> 子数组 [2,3] 和的绝对值最大，为 abs(2+3) = abs(5) = 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,-5,1,-4,3,-2]
<strong>输出:</strong> 8
<strong>解释:</strong> 子数组 [-5,1,-4] 和的绝对值最大，为 abs(-5+1-4) = abs(-8) = 8 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Ruby)

### 1. 前缀和
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

## 题解 (Rust)

### 1. 前缀和
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
