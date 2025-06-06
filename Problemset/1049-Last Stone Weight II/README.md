# 1049. Last Stone Weight II
You are given an array of integers `stones` where `stones[i]` is the weight of the <code>i<sup>th</sup></code> stone.

We are playing a game with the stones. On each turn, we choose any two stones and smash them together. Suppose the stones have weights `x` and `y` with `x <= y`. The result of this smash is:
* If `x == y`, both stones are destroyed, and
* If `x != y`, the stone of weight `x` is destroyed, and the stone of weight `y` has new weight `y - x`.

At the end of the game, there is **at most one** stone left.

Return *the smallest possible weight of the left stone*. If there are no stones left, return `0`.

#### Example 1:
<pre>
<strong>Input:</strong> stones = [2,7,4,1,8,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
We can combine 2 and 4 to get 2, so the array converts to [2,7,1,8,1] then,
we can combine 7 and 8 to get 1, so the array converts to [2,1,1,1] then,
we can combine 2 and 1 to get 1, so the array converts to [1,1,1] then,
we can combine 1 and 1 to get 0, so the array converts to [1], then that's the optimal value.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stones = [31,26,33,21,40]
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `1 <= stones.length <= 30`
* `1 <= stones[i] <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def lastStoneWeightII(self, stones: List[int]) -> int:
        @cache
        def dfs(i: int, target: int) -> int:
            if i == len(stones):
                return target

            return min(dfs(i + 1, abs(target - stones[i])), dfs(i + 1, abs(target + stones[i])))

        return dfs(0, 0)
```
