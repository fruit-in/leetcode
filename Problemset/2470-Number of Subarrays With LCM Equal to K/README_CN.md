# 2470. 最小公倍数等于 K 的子数组数目
给你一个整数数组 `nums` 和一个整数 `k` ，请你统计并返回 `nums` 的 **子数组** 中满足 *元素最小公倍数为 `k`* 的子数组数目。

**子数组** 是数组中一个连续非空的元素序列。

**数组的最小公倍数** 是可被所有数组元素整除的最小正整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,6,2,7,1], k = 6
<strong>输出:</strong> 4
<strong>解释:</strong> 以 6 为最小公倍数的子数组是：
- [3,6,2,7,1]
- [3,6,2,7,1]
- [3,6,2,7,1]
- [3,6,2,7,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3], k = 2
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在以 2 为最小公倍数的子数组。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i], k <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def subarrayLCM(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            x = 1
            for j in range(i, len(nums)):
                x = lcm(x, nums[j])
                if x == k:
                    ret += 1
                elif x > k:
                    break

        return ret
```
