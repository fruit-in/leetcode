# 55. 跳跃游戏
给定一个非负整数数组 `nums` ，你最初位于数组的 **第一个下标** 。

数组中的每个元素代表你在该位置可以跳跃的最大长度。

判断你是否能够到达最后一个下标。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,1,1,4]
<strong>输出:</strong> true
<strong>解释:</strong> 可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,2,1,0,4]
<strong>输出:</strong> false
<strong>解释:</strong> 无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。
</pre>

#### 提示:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Ruby)

### 1. 贪心
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

## 题解 (Rust)

### 1. 贪心
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
