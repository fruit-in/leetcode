# 2529. 正整数和负整数的最大计数
给你一个按 **非递减顺序** 排列的数组 `nums` ，返回正整数数目和负整数数目中的最大值。

* 换句话讲，如果 `nums` 中正整数的数目是 `pos` ，而负整数的数目是 `neg` ，返回 `pos` 和 `neg`二者中的最大值。

**注意：**`0` 既不是正整数也不是负整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-2,-1,-1,1,2,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 共有 3 个正整数和 3 个负整数。计数得到的最大值是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-3,-2,-1,0,0,1,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 共有 2 个正整数和 3 个负整数。计数得到的最大值是 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [5,20,66,1314]
<strong>输出:</strong> 4
<strong>解释:</strong> 共有 4 个正整数和 0 个负整数。计数得到的最大值是 4 。
</pre>

#### 提示:
* `1 <= nums.length <= 2000`
* `-2000 <= nums[i] <= 2000`
* `nums` 按 **非递减顺序** 排列。

**进阶：**你可以设计并实现时间复杂度为 `O(log(n))` 的算法解决此问题吗？

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximumCount(self, nums: List[int]) -> int:
        pos = bisect.bisect_left(nums, 0)
        neg = len(nums) - bisect.bisect_right(nums, 0)

        return max(pos, neg)
```
