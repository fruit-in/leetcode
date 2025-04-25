# 1140. Stone Game II
Alice and Bob continue their games with piles of stones. There are a number of piles **arranged in a row**, and each pile has a positive integer number of stones `piles[i]`. The objective of the game is to end with the most stones.

Alice and Bob take turns, with Alice starting first.

On each player's turn, that player can take **all the stones** in the **first** X remaining piles, where `1 <= X <= 2M`. Then, we set `M = max(M, X)`. Initially, M = 1.

The game continues until all the stones have been taken.

Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.

#### Example 1:
<pre>
<strong>Input:</strong> piles = [2,7,9,4,4]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
* If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 stones in total.
* If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 stones in total.
So we return 10 since it's larger.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> piles = [1,2,3,4,5,100]
<strong>Output:</strong> 104
</pre>

#### Constraints:
* `1 <= piles.length <= 100`
* <code>1 <= piles[i] <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def stoneGameII(self, piles: List[int]) -> int:
        @cache
        def subGame(m: int, i: int) -> int:
            if i == len(piles):
                return 0

            total = sum(piles[i:])

            return total - min(subGame(max(m, x), i + x) for x in range(1, min(2 * m, len(piles) - i) + 1))

        return subGame(1, 0)
```
