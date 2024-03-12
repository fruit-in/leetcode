# 805. 数组的均值分割
给定你一个整数数组 `nums`

我们要将 `nums` 数组中的每个元素移动到 `A` 数组 或者 `B` 数组中，使得 `A` 数组和 `B` 数组不为空，并且 `average(A) == average(B)` 。

如果可以完成则返回`true` ， 否则返回 `false`  。

**注意：**对于数组 `arr` ,  `average(arr)` 是 `arr` 的所有元素的和除以 `arr` 长度。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6,7,8]
<strong>输出:</strong> true
<strong>解释:</strong> 我们可以将数组分割为 [1,4,5,8] 和 [2,3,6,7], 他们的平均值都是4.5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,1]
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= nums.length <= 30`
* <code>0 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def splitArraySameAverage(self, nums: List[int]) -> bool:
        if len(nums) < 2:
            return False

        s = sum(nums)
        sums = [set() for _ in range(len(nums) // 2 + 1)]
        sums[0].add(0)

        for num in nums:
            for i in range(len(sums) - 2, -1, -1):
                for x in sums[i]:
                    if (x + num) * len(nums) == s * (i + 1):
                        return True

                    sums[i + 1].add(x + num)

        return False
```
