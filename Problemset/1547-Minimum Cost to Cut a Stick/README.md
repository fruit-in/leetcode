# 1547. Minimum Cost to Cut a Stick
Given a wooden stick of length `n` units. The stick is labelled from `0` to `n`. For example, a stick of length **6** is labelled as follows:

![](https://assets.leetcode.com/uploads/2020/07/21/statement.jpg)

Given an integer array `cuts` where `cuts[i]` denotes a position you should perform a cut at.

You should perform the cuts in order, you can change the order of the cuts as you wish.

The cost of one cut is the length of the stick to be cut, the total cost is the sum of costs of all cuts. When you cut a stick, it will be split into two smaller sticks (i.e. the sum of their lengths is the length of the stick before the cut). Please refer to the first example for a better explanation.

Return *the minimum total cost* of the cuts.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/07/23/e1.jpg)
<pre>
<strong>Input:</strong> n = 7, cuts = [1,3,4,5]
<strong>Output:</strong> 16
<strong>Explanation:</strong> Using cuts order = [1, 3, 4, 5] as in the input leads to the following scenario:
<img src="https://assets.leetcode.com/uploads/2020/07/21/e11.jpg">
The first cut is done to a rod of length 7 so the cost is 7. The second cut is done to a rod of length 6 (i.e. the second part of the first cut), the third is done to a rod of length 4 and the last cut is to a rod of length 3. The total cost is 7 + 6 + 4 + 3 = 20.
Rearranging the cuts to be [3, 5, 1, 4] for example will lead to a scenario with total cost = 16 (as shown in the example photo 7 + 4 + 3 + 2 = 16).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 9, cuts = [5,6,1,4,2]
<strong>Output:</strong> 22
<strong>Explanation:</strong> If you try the given cuts ordering the cost will be 25.
There are much ordering with total cost <= 25, for example, the order [4, 6, 5, 2, 1] has total cost = 22 which is the minimum possible.
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>6</sup></code>
* `1 <= cuts.length <= min(n - 1, 100)`
* `1 <= cuts[i] <= n - 1`
* All the integers in `cuts` array are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def minCost(self, n: int, cuts: List[int]) -> int:
        @cache
        def subStickMinCost(i: int, j: int) -> int:
            if i > j:
                return 0

            left, right = 0, n
            if i > 0:
                left = cuts[i - 1]
            if j < len(cuts) - 1:
                right = cuts[j + 1]

            return right - left + min(subStickMinCost(i, k - 1) + subStickMinCost(k + 1, j) for k in range(i, j + 1))

        cuts.sort()

        return subStickMinCost(0, len(cuts) - 1)
```
