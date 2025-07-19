# 2616. 最小化数对的最大差值
给你一个下标从 **0** 开始的整数数组 `nums` 和一个整数 `p` 。请你从 `nums` 中找到 `p` 个下标对，每个下标对对应数值取差值，你需要使得这 `p` 个差值的 **最大值 最小**。同时，你需要确保每个下标在这 `p` 个下标对中最多出现一次。

对于一个下标对 `i` 和 `j` ，这一对的差值为 `|nums[i] - nums[j]|` ，其中 `|x|` 表示 `x` 的 **绝对值** 。

请你返回 `p` 个下标对对应数值 **最大差值** 的 **最小值** 。我们定义空集的最大值为零。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,1,2,7,1,3], p = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 第一个下标对选择 1 和 4 ，第二个下标对选择 2 和 5 。
最大差值为 max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1 。所以我们返回 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,2,1,2], p = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 选择下标 1 和 3 构成下标对。差值为 |2 - 2| = 0 ，这是最大差值的最小值。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* `0 <= p <= (nums.length)/2`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minimizeMax(self, nums: List[int], p: int) -> int:
        def countPairs(maxdiff: int) -> int:
            i = 0
            ret = 0

            while i < len(diffs):
                if diffs[i] <= maxdiff:
                    ret += 1
                    i += 1
                i += 1

            return ret

        nums.sort()
        diffs = [nums[i + 1] - nums[i] for i in range(len(nums) - 1)]

        return bisect_left(range(0, max(diffs, default=0) + 1), p, key=countPairs)
```
