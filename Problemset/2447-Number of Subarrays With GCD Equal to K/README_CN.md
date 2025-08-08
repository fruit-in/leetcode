# 2447. 最大公因数等于 K 的子数组数目
给你一个整数数组 `nums` 和一个整数 `k` ，请你统计并返回 `nums` 的子数组中元素的最大公因数等于 `k` 的子数组数目。

**子数组** 是数组中一个连续的非空序列。

**数组的最大公因数** 是能整除数组中所有元素的最大整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [9,3,1,2,6,3], k = 3
<strong>输出:</strong> 4
<strong>解释:</strong> nums 的子数组中，以 3 作为最大公因数的子数组如下：
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4], k = 7
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在以 7 作为最大公因数的子数组。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i], k <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def subarrayGCD(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            maxgcd = nums[i]

            for j in range(i, len(nums)):
                maxgcd = gcd(maxgcd, nums[j])
                if maxgcd % k != 0:
                    break
                elif maxgcd == k:
                    ret += 1

        return ret
```
