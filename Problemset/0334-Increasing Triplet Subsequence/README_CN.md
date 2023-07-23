# 334. 递增的三元子序列
给你一个整数数组 `nums` ，判断这个数组中是否存在长度为 `3` 的递增子序列。

如果存在这样的三元组下标 `(i, j, k)` 且满足 `i < j < k` ，使得 `nums[i] < nums[j] < nums[k]` ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> true
<strong>解释:</strong> 任何 i < j < k 的三元组都满足题意
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,4,3,2,1]
<strong>输出:</strong> false
<strong>解释:</strong> 不存在满足题意的三元组
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,1,5,0,4,6]
<strong>输出:</strong> true
<strong>解释:</strong> 三元组 (3, 4, 5) 满足题意，因为 nums[3] == 0 < nums[4] == 4 < nums[5] == 6
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

**进阶:** 你能实现时间复杂度为 `O(n)` ，空间复杂度为 `O(1)` 的解决方案吗？

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Boolean}
def increasing_triplet(nums)
  min3 = [nil, nil, nil]

  nums.each do |x|
    if min3[1].nil?
      min3[1] = x
    elsif min3[2].nil? && x <= min3[1]
      min3[1] = x
    elsif min3[2].nil?
      min3[2] = x
    elsif x > min3[2]
      return true
    elsif x > min3[1]
      min3[2] = x
    elsif min3[0].nil? && x < min3[1]
      min3[0] = x
    elsif !min3[0].nil? && x > min3[0]
      min3 = [nil, min3[0], x]
    elsif !min3[0].nil? && x < min3[0]
      min3[0] = x
    end
  end

  false
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min3 = (None, None, None);

        for x in nums {
            match min3 {
                (_, None, _) => min3.1 = Some(x),
                (_, Some(b), None) if x < b => min3.1 = Some(x),
                (_, Some(b), None) if x > b => min3.2 = Some(x),
                (_, Some(b), Some(c)) if x > c => return true,
                (_, Some(b), Some(c)) if x > b => min3.2 = Some(x),
                (None, Some(b), Some(c)) if x < b => min3.0 = Some(x),
                (Some(a), Some(b), Some(c)) if x > a => min3 = (None, Some(a), Some(x)),
                (Some(a), Some(b), Some(c)) if x < a => min3.0 = Some(x),
                _ => (),
            }
        }

        false
    }
}
```
