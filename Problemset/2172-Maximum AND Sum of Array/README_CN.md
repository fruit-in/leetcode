# 2172. 数组的最大与和
给你一个长度为 `n` 的整数数组 `nums` 和一个整数 `numSlots` ，满足`2 * numSlots >= n` 。总共有 `numSlots` 个篮子，编号为 `1` 到 `numSlots` 。

你需要把所有 `n` 个整数分到这些篮子中，且每个篮子 **至多** 有 2 个整数。一种分配方案的 **与和** 定义为每个数与它所在篮子编号的 **按位与运算** 结果之和。

* 比方说，将数字 `[1, 3]` 放入篮子 *`1`* 中，`[4, 6]` 放入篮子 *`2`* 中，这个方案的与和为 `(1 AND 1) + (3 AND 1) + (4 AND 2) + (6 AND 2) = 1 + 1 + 0 + 2 = 4` 。

请你返回将 `nums` 中所有数放入 `numSlots` 个篮子中的最大与和。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6], numSlots = 3
<strong>输出:</strong> 9
<strong>解释:</strong> 一个可行的方案是 [1, 4] 放入篮子 1 中，[2, 6] 放入篮子 2 中，[3, 5] 放入篮子 3 中。
最大与和为 (1 AND 1) + (4 AND 1) + (2 AND 2) + (6 AND 2) + (3 AND 3) + (5 AND 3) = 1 + 0 + 2 + 2 + 3 + 1 = 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,10,4,7,1], numSlots = 9
<strong>输出:</strong> 24
<strong>解释:</strong> 一个可行的方案是 [1, 1] 放入篮子 1 中，[3] 放入篮子 3 中，[4] 放入篮子 4 中，[7] 放入篮子 7 中，[10] 放入篮子 9 中。
最大与和为 (1 AND 1) + (1 AND 1) + (3 AND 3) + (4 AND 4) + (7 AND 7) + (10 AND 9) = 1 + 1 + 3 + 4 + 7 + 8 = 24 。
注意，篮子 2 ，5 ，6 和 8 是空的，这是允许的。
</pre>

#### 提示:
* `n == nums.length`
* `1 <= numSlots <= 9`
* `1 <= n <= 2 * numSlots`
* `1 <= nums[i] <= 15`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def maximumANDSum(self, nums: List[int], numSlots: int) -> int:
        @cache
        def dfs(i: int, slots: Tuple[int]) -> int:
            if i == len(nums):
                return 0

            slots = list(slots)
            ret = 0

            for j in range(numSlots):
                if slots[j] < 2:
                    slots[j] += 1
                    ret = max(ret, (nums[i] & (j + 1)) +
                              dfs(i + 1, tuple(slots)))
                    slots[j] -= 1

            return ret

        return dfs(0, tuple([0] * numSlots))
```
