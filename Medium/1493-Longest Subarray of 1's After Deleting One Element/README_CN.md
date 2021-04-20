# 1493. 删掉一个元素以后全为 1 的最长子数组
给你一个二进制数组 `nums` ，你需要从中删掉一个元素。

请你在删掉元素的结果数组中，返回最长的且只包含 1 的非空子数组的长度。

如果不存在这样的子数组，请返回 0 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,0,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 删掉位置 2 的数后，[1,1,1] 包含 3 个 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,1,1,1,0,1,1,0,1]
<strong>输出:</strong> 5
<strong>解释:</strong> 删掉位置 4 的数字后，[0,1,1,1,1,1,0,1] 的最长全 1 子数组为 [1,1,1,1,1] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> 2
<strong>解释:</strong> 你必须要删除一个元素。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1,1,0,0,1,1,1,0,1]
<strong>输出:</strong> 4
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> nums = [0,0,0]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `nums[i]` 要么是 `0` 要么是 `1` 。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def longest_subarray(nums)
  zeros = 0
  l = 0
  ret = 0

  (0...nums.size).each do |r|
    zeros += 1 if nums[r] == 0
    while zeros > 1
      zeros -= 1 if nums[l] == 0
      l += 1
    end
    ret = [ret, r - l - zeros + 1].max
  end

  [ret, nums.size - 1].min
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut l = 0;
        let mut ret = 0;

        for r in 0..nums.len() {
            if nums[r] == 0 {
                zeros += 1;
            }
            while zeros > 1 {
                if nums[l] == 0 {
                    zeros -= 1;
                }
                l += 1;
            }
            ret = ret.max(r - l - zeros + 1)
        }

        ret.min(nums.len() - 1) as i32
    }
}
```
