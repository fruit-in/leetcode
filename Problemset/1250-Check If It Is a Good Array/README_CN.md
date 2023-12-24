# 1250. 检查「好数组」
给你一个正整数数组 `nums`，你需要从中任选一些子集，然后将子集中每一个数乘以一个 **任意整数**，并求出他们的和。

假如该和结果为 `1`，那么原数组就是一个「**好数组**」，则返回 `True`；否则请返回 `False`。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [12,5,7,23]
<strong>输出:</strong> true
<strong>解释:</strong> 挑选数字 5 和 7。
5*3 + 7*(-2) = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [29,6,10]
<strong>输出:</strong> true
<strong>解释:</strong> 挑选数字 29, 6 和 10。
29*1 + 6*(-3) + 10*(-1) = 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,6]
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= nums.length <= 10^5`
* `1 <= nums[i] <= 10^9`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def isGoodArray(self, nums: List[int]) -> bool:
        return math.gcd(*nums) == 1
```
