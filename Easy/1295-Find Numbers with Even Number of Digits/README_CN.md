# 1295. 统计位数为偶数的数字
给你一个整数数组 ```nums```，请你返回其中位数为 **偶数** 的数字的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [12,345,2,6,7896]
<strong>输出:</strong> 2
<strong>输出:</strong>
12 是 2 位数字（位数为偶数）
345 是 3 位数字（位数为奇数）
2 是 1 位数字（位数为奇数）
6 是 1 位数字 位数为奇数）
7896 是 4 位数字（位数为偶数）
因此只有 12 和 7896 是位数为偶数的数字
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [555,901,482,1771]
<strong>输出:</strong> 1
<strong>输出:</strong>
只有 1771 是位数为偶数的数字。
</pre>

#### 提示:
* ```1 <= nums.length <= 500```
* ```1 <= nums[i] <= 10^5```

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        return sum(1 for n in nums if len(str(n)) % 2 == 0)
```
