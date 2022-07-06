# 1991. 找到数组的中间位置
给你一个下标从 **0** 开始的整数数组 `nums` ，请你找到 **最左边** 的中间位置 `middleIndex` （也就是所有可能中间位置下标最小的一个）。

中间位置 `middleIndex` 是满足 `nums[0] + nums[1] + ... + nums[middleIndex-1] == nums[middleIndex+1] + nums[middleIndex+2] + ... + nums[nums.length-1]` 的数组下标。

如果 `middleIndex == 0` ，左边部分的和定义为 `0` 。类似的，如果 `middleIndex == nums.length - 1` ，右边部分的和定义为 `0` 。

请你返回满足上述条件 **最左边** 的 `middleIndex` ，如果不存在这样的中间位置，请你返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,-1,8,4]
<strong>输出:</strong> 3
<strong>解释:</strong>
下标 3 之前的数字和为：2 + 3 + -1 = 4
下标 3 之后的数字和为：4 = 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,-1,4]
<strong>输出:</strong> 2
<strong>解释:</strong>
下标 2 之前的数字和为：1 + -1 = 0
下标 2 之后的数字和为：0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,5]
<strong>输出:</strong> -1
<strong>解释:</strong>
不存在符合要求的 middleIndex 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [1]
<strong>输出:</strong> 0
<strong>解释:</strong>
下标 0 之前的数字和为：0
下标 0 之后的数字和为：0
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `-1000 <= nums[i] <= 1000`

**注意:** 本题与主站 724 题相同：https://leetcode-cn.com/problems/find-pivot-index/

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findMiddleIndex(self, nums: List[int]) -> int:
        total_sum = sum(nums)
        left_sum = 0

        for i in range(len(nums)):
            if left_sum * 2 + nums[i] == total_sum:
                return i
            left_sum += nums[i]

        return -1
```
