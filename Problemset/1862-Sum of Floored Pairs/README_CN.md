# 1862. 向下取整数对和
给你一个整数数组 `nums` ，请你返回所有下标对 `0 <= i, j < nums.length` 的 `floor(nums[i] / nums[j])` 结果之和。由于答案可能会很大，请你返回答案对<code>10<sup>9</sup> + 7</code> **取余** 的结果。

函数 `floor()` 返回输入数字的整数部分。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,5,9]
<strong>输出:</strong> 10
<strong>解释:</strong>
floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
floor(5 / 2) = 2
floor(9 / 2) = 4
floor(9 / 5) = 1
我们计算每一个数对商向下取整的结果并求和得到 10 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [7,7,7,7,7,7,7]
<strong>输出:</strong> 49
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def sumOfFlooredPairs(self, nums: List[int]) -> int:
        prevsum = 0
        ret = 0

        nums.sort()

        for j in range(len(nums)):
            if j > 0 and nums[j] == nums[j - 1]:
                ret = (ret + prevsum) % 1000000007
                continue

            i = bisect.bisect(nums, nums[j] - 1)
            prevsum = 0

            for x in range(1, nums[-1] // nums[j] + 1):
                k = bisect.bisect(nums, nums[j] * (x + 1) - 1, lo=i)
                prevsum = (prevsum + x * (k - i)) % 1000000007
                i = k

            ret = (ret + prevsum) % 1000000007

        return ret
```
