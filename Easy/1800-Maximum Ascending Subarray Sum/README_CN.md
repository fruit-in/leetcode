# 1800. 最大升序子数组和
给你一个正整数组成的数组 `nums` ，返回 `nums` 中一个 **升序** 子数组的最大可能元素和。

子数组是数组中的一个连续数字序列。

已知子数组 <code>[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]</code> ，若对所有 `i`（`l <= i < r`），<code>nums<sub>i</sub> < nums<sub>i+1</sub></code> 都成立，则称这一子数组为 **升序** 子数组。注意，大小为 `1` 的子数组也视作 **升序** 子数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,20,30,5,10,50]
<strong>输出:</strong> 65
<strong>解释:</strong> [5,10,50] 是元素和最大的升序子数组，最大元素和为 65 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10,20,30,40,50]
<strong>输出:</strong> 150
<strong>解释:</strong> [10,20,30,40,50] 是元素和最大的升序子数组，最大元素和为 150 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [12,17,15,13,10,11,12]
<strong>输出:</strong> 33
<strong>解释:</strong> [10,11,12] 是元素和最大的升序子数组，最大元素和为 33 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [100,10,1]
<strong>输出:</strong> 100
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def max_ascending_sum(nums)
  sum = nums[0]
  ret = sum

  (1...nums.size).each do |i|
    sum = nums[i] + (nums[i] > nums[i - 1] ? sum : 0)
    ret = [ret, sum].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut ret = sum;

        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            ret = ret.max(sum);
        }

        ret
    }
}
```
