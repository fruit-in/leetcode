# 1695. 删除子数组的最大得分
给你一个正整数数组 `nums` ，请你从中删除一个含有 **若干不同元素** 的子数组。删除子数组的 **得分** 就是子数组各元素之 **和** 。

返回 **只删除一个** 子数组可获得的 **最大得分** 。

如果数组 `b` 是数组 `a` 的一个连续子序列，即如果它等于 `a[l],a[l+1],...,a[r]` ，那么它就是 `a` 的一个子数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,2,4,5,6]
<strong>输出:</strong> 17
<strong>解释:</strong> 最优子数组是 [2,4,5,6]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,2,1,2,5,2,1,2,5]
<strong>输出:</strong> 8
<strong>解释:</strong> 最优子数组是 [5,2,1] 或 [1,2,5]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def maximum_unique_subarray(nums)
  i = 0
  counter = {}
  counter.default = 0
  sum = 0
  ret = 0

  (0...nums.size).each do |j|
    counter[nums[j]] += 1
    sum += nums[j]
    while counter[nums[j]] > 1
      counter[nums[i]] -= 1
      sum -= nums[i]
      i += 1
    end
    ret = [ret, sum].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut counter = HashMap::new();
        let mut sum = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            *counter.entry(nums[j]).or_insert(0) += 1;
            sum += nums[j];
            while *counter.get(&nums[j]).unwrap() > 1 {
                *counter.get_mut(&nums[i]).unwrap() -= 1;
                sum -= nums[i];
                i += 1;
            }
            ret = ret.max(sum);
        }

        ret
    }
}
```
