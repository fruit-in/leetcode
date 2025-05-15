# 1655. 分配重复整数
给你一个长度为 `n` 的整数数组 `nums` ，这个数组中至多有 `50` 个不同的值。同时你有 `m` 个顾客的订单 `quantity` ，其中，整数 `quantity[i]` 是第 `i` 位顾客订单的数目。请你判断是否能将 `nums` 中的整数分配给这些顾客，且满足：
* 第 `i` 位顾客 **恰好** 有 `quantity[i]` 个整数。
* 第 `i` 位顾客拿到的整数都是 **相同的** 。
* 每位顾客都满足上述两个要求。

如果你可以分配 `nums` 中的整数满足上面的要求，那么请返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], quantity = [2]
<strong>输出:</strong> false
<strong>解释:</strong> 第 0 位顾客没办法得到两个相同的整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,3], quantity = [2]
<strong>输出:</strong> true
<strong>解释:</strong> 第 0 位顾客得到 [3,3] 。整数 [1,2] 都没有被使用。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,2,2], quantity = [2,2]
<strong>输出:</strong> true
<strong>解释:</strong> 第 0 位顾客得到 [1,1] ，第 1 位顾客得到 [2,2] 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 1000`
* `m == quantity.length`
* `1 <= m <= 10`
* <code>1 <= quantity[i] <= 10<sup>5</sup></code>
* `nums` 中至多有 `50` 个不同的数字。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def canDistribute(self, nums: List[int], quantity: List[int]) -> bool:
        def dfs(i: int) -> bool:
            if i == len(quantity):
                return True

            for j in range(len(count)):
                if count[j] >= quantity[i] and (j == 0 or count[j] != count[j - 1]):
                    count[j] -= quantity[i]
                    if dfs(i + 1):
                        return True
                    count[j] += quantity[i]

            return False

        count = sorted(collections.Counter(nums).values())[-len(quantity):]

        if count[-1] < max(quantity) or sum(count) < sum(quantity):
            return False

        return dfs(0)
```
