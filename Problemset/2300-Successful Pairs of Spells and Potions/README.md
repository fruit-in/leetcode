# 2300. Successful Pairs of Spells and Potions
You are given two positive integer arrays `spells` and `potions`, of length `n` and `m` respectively, where `spells[i]` represents the strength of the <code>i<sup>th</sup></code> spell and `potions[j]` represents the strength of the <code>j<sup>th</sup></code> potion.

You are also given an integer `success`. A spell and potion pair is considered **successful** if the **product** of their strengths is **at least** `success`.

Return *an integer array* `pairs` *of length* `n` *where* `pairs[i]` *is the number of **potions** that will form a successful pair with the* <code>i<sup>th</sup></code> *spell*.

#### Example 1:
<pre>
<strong>Input:</strong> spells = [5,1,3], potions = [1,2,3,4,5], success = 7
<strong>Output:</strong> [4,0,3]
<strong>Explanation:</strong>
- 0th spell: 5 * [1,2,3,4,5] = [5,10,15,20,25]. 4 pairs are successful.
- 1st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
- 2nd spell: 3 * [1,2,3,4,5] = [3,6,9,12,15]. 3 pairs are successful.
Thus, [4,0,3] is returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> spells = [3,1,2], potions = [8,5,8], success = 16
<strong>Output:</strong> [2,0,2]
<strong>Explanation:</strong>
- 0th spell: 3 * [8,5,8] = [24,15,24]. 2 pairs are successful.
- 1st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
- 2nd spell: 2 * [8,5,8] = [16,10,16]. 2 pairs are successful.
Thus, [2,0,2] is returned.
</pre>

#### Constraints:
* `n == spells.length`
* `m == potions.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* <code>1 <= spells[i], potions[i] <= 10<sup>5</sup></code>
* <code>1 <= success <= 10<sup>10</sup></code>

## Solutions (Python)

### 1. Solution
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
