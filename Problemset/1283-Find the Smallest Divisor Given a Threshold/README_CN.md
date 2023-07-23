# 1283. 使结果不超过阈值的最小除数
给你一个整数数组 `nums` 和一个正整数 `threshold`  ，你需要选择一个正整数作为除数，然后将数组里每个数都除以它，并对除法结果求和。

请你找出能够使上述结果小于等于阈值 `threshold` 的除数中 **最小** 的那个。

每个数除以除数后都向上取整，比方说 `7/3 = 3` ， `10/2 = 5` 。

题目保证一定有解。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,5,9], threshold = 6
<strong>输出:</strong> 5
<strong>解释:</strong> 如果除数为 1 ，我们可以得到和为 17 （1+2+5+9）。
如果除数为 4 ，我们可以得到和为 7 (1+1+2+3) 。如果除数为 5 ，和为 5 (1+1+1+2)。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,5,7,11], threshold = 11
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [19], threshold = 5
<strong>输出:</strong> 4
</pre>

#### 提示:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* <code>nums.length <= threshold <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
```Python
import bisect
import math


class Solution:
    def smallestDivisor(self, nums: List[int], threshold: int) -> int:
        divisors = list(range(1, max(nums) + 1))

        return 1 + bisect.bisect_left(
            divisors,
            True,
            key=lambda x: sum(math.ceil(num / x) for num in nums) <= threshold
        )
```
