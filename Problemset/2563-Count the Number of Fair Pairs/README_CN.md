# 2563. 统计公平数对的数目
给你一个下标从 **0** 开始、长度为 `n` 的整数数组 `nums` ，和两个整数 `lower` 和 `upper` ，返回 **公平数对的数目** 。

如果 `(i, j)` 数对满足以下情况，则认为它是一个 **公平数对** ：

* `0 <= i < j < n`，且
* `lower <= nums[i] + nums[j] <= upper`

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,7,4,4,5], lower = 3, upper = 6
<strong>输出:</strong> 6
<strong>解释:</strong> 共计 6 个公平数对：(0,3)、(0,4)、(0,5)、(1,3)、(1,4) 和 (1,5) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,7,9,2,5], lower = 11, upper = 11
<strong>输出:</strong> 1
<strong>解释:</strong> 只有单个公平数对：(2,3) 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums.length == n`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= lower <= upper <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()
        ret = 0

        for i in range(len(nums) - 1):
            if nums[i] + nums[-1] < lower:
                continue
            if nums[i] + nums[i + 1] > upper:
                break

            j = max(bisect.bisect_left(nums, lower - nums[i]), i + 1)
            k = bisect.bisect(nums, upper - nums[i])
            ret += k - j

        return ret
```
