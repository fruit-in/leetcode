# 473. Matchsticks to Square
You are given an integer array `matchsticks` where `matchsticks[i]` is the length of the <code>i<sup>th</sup></code> matchstick. You want to use **all the matchsticks** to make one square. You **should not break** any stick, but you can link them up, and each matchstick must be used **exactly one time**.

Return `true` if you can make this square and `false` otherwise.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/04/09/matchsticks1-grid.jpg)
<pre>
<strong>Input:</strong> matchsticks = [1,1,2,2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> You can form a square with length 2, one side of the square came two sticks with length 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matchsticks = [3,3,3,3,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> You cannot find a way to form a square with all the matchsticks.
</pre>

#### Constraints:
* `1 <= matchsticks.length <= 15`
* <code>1 <= matchsticks[i] <= 10<sup>8</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def makesquare(self, matchsticks: List[int]) -> bool:
        perimeter = sum(matchsticks)

        if perimeter % 4 != 0:
            return False

        combs = {(0, 0, 0, 0)}

        for stick in matchsticks:
            newcombs = set()

            for comb in combs:
                for i in range(4):
                    if comb[i] + stick <= perimeter // 4:
                        newcomb = list(comb)
                        newcomb[i] += stick
                        newcombs.add(tuple(sorted(newcomb)))

            combs = newcombs

        return len(combs) > 0
```
