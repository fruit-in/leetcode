# 2300. 咒语和药水的成功对数
给你两个正整数数组 `spells` 和 `potions` ，长度分别为 `n` 和 `m` ，其中 `spells[i]` 表示第 `i` 个咒语的能量强度，`potions[j]` 表示第 `j` 瓶药水的能量强度。

同时给你一个整数 `success` 。一个咒语和药水的能量强度 **相乘** 如果 **大于等于** `success` ，那么它们视为一对 **成功** 的组合。

请你返回一个长度为 `n` 的整数数组 `pairs`，其中 `pairs[i]` 是能跟第 `i` 个咒语成功组合的 **药水** 数目。

#### 示例 1:
<pre>
<strong>输入:</strong> spells = [5,1,3], potions = [1,2,3,4,5], success = 7
<strong>输出:</strong> [4,0,3]
<strong>解释:</strong>
- 第 0 个咒语：5 * [1,2,3,4,5] = [5,10,15,20,25] 。总共 4 个成功组合。
- 第 1 个咒语：1 * [1,2,3,4,5] = [1,2,3,4,5] 。总共 0 个成功组合。
- 第 2 个咒语：3 * [1,2,3,4,5] = [3,6,9,12,15] 。总共 3 个成功组合。
所以返回 [4,0,3] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> spells = [3,1,2], potions = [8,5,8], success = 16
<strong>输出:</strong> [2,0,2]
<strong>解释:</strong>
- 第 0 个咒语：3 * [8,5,8] = [24,15,24] 。总共 2 个成功组合。
- 第 1 个咒语：1 * [8,5,8] = [8,5,8] 。总共 0 个成功组合。
- 第 2 个咒语：2 * [8,5,8] = [16,10,16] 。总共 2 个成功组合。
所以返回 [2,0,2] 。
</pre>

#### 提示:
* `n == spells.length`
* `m == potions.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* <code>1 <= spells[i], potions[i] <= 10<sup>5</sup></code>
* <code>1 <= success <= 10<sup>10</sup></code>

## 题解 (Python)

### 1. 题解
```Python
import bisect


class Solution:
    def successfulPairs(self, spells: List[int], potions: List[int], success: int) -> List[int]:
        pairs = [len(potions)] * len(spells)
        potions.sort()

        for i in range(len(spells)):
            pairs[i] -= bisect.bisect_left(potions, True,
                                           key=lambda x: spells[i] * x >= success)

        return pairs
```
