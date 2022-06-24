# 1929. 数组串联
给你一个长度为 `n` 的整数数组 `nums` 。请你构建一个长度为 `2n` 的答案数组 `ans` ，数组下标 **从 0 开始计数** ，对于所有 `0 <= i < n` 的 `i` ，满足下述所有要求：
* `ans[i] == nums[i]`
* `ans[i + n] == nums[i]`

具体而言，`ans` 由两个 `nums` 数组 **串联** 形成。

返回数组 `ans` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1]
<strong>输出:</strong> [1,2,1,1,2,1]
<strong>解释:</strong> 数组 ans 按下述方式形成：
- ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
- ans = [1,2,1,1,2,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,2,1]
<strong>输出:</strong> [1,3,2,1,1,3,2,1]
<strong>解释:</strong> 数组 ans 按下述方式形成：
- ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
- ans = [1,3,2,1,1,3,2,1]
</pre>

#### 提示:
* `n == nums.length`
* `1 <= n <= 1000`
* `1 <= nums[i] <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getConcatenation(self, nums: List[int]) -> List[int]:
        return nums + nums
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        nums.clone().into_iter().chain(nums.into_iter()).collect()
    }
}
```
