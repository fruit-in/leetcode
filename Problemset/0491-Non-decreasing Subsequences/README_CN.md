# 491. 非递减子序列
给你一个整数数组 `nums` ，找出并返回所有该数组中不同的递增子序列，递增子序列中 **至少有两个元素** 。你可以按 **任意顺序** 返回答案。

数组中可能含有重复元素，如出现两个整数相等，也可以视作递增序列的一种特殊情况。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,6,7,7]
<strong>输出:</strong> [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,4,3,2,1]
<strong>输出:</strong> [[4,4]]
</pre>

#### 提示:
* `1 <= nums.length <= 15`
* `-100 <= nums[i] <= 100`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        def dfs(i: int) -> None:
            if i == len(nums):
                if len(sub) >= 2:
                    subsequences.add(tuple(sub))
                return

            if not sub or nums[i] >= sub[-1]:
                sub.append(nums[i])
                dfs(i + 1)
                sub.pop()
            dfs(i + 1)

        sub = []
        subsequences = set()
        dfs(0)

        return [list(sub) for sub in subsequences]
```
