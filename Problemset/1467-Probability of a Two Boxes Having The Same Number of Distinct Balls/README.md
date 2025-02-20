# 1467. Probability of a Two Boxes Having The Same Number of Distinct Balls
Given `2n` balls of `k` distinct colors. You will be given an integer array `balls` of size `k` where `balls[i]` is the number of balls of color `i`.

All the balls will be **shuffled uniformly at random**, then we will distribute the first `n` balls to the first box and the remaining `n` balls to the other box (Please read the explanation of the second example carefully).

Please note that the two boxes are considered different. For example, if we have two balls of colors `a` and `b`, and two boxes `[]` and `()`, then the distribution `[a] (b)` is considered different than the distribution `[b] (a)` (Please read the explanation of the first example carefully).

Return *the probability* that the two boxes have the same number of distinct balls. Answers within <code>10<sup>-5</sup></code> of the actual value will be accepted as correct.

#### Example 1:
<pre>
<strong>Input:</strong> balls = [1,1]
<strong>Output:</strong> 1.00000
<strong>Explanation:</strong> Only 2 ways to divide the balls equally:
- A ball of color 1 to box 1 and a ball of color 2 to box 2
- A ball of color 2 to box 1 and a ball of color 1 to box 2
In both ways, the number of distinct colors in each box is equal. The probability is 2/2 = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> balls = [2,1,1]
<strong>Output:</strong> 0.66667
<strong>Explanation:</strong> We have the set of balls [1, 1, 2, 3]
This set of balls will be shuffled randomly and we may have one of the 12 distinct shuffles with equal probability (i.e. 1/12):
[1,1 / 2,3], [1,1 / 3,2], [1,2 / 1,3], [1,2 / 3,1], [1,3 / 1,2], [1,3 / 2,1], [2,1 / 1,3], [2,1 / 3,1], [2,3 / 1,1], [3,1 / 1,2], [3,1 / 2,1], [3,2 / 1,1]
After that, we add the first two balls to the first box and the second two balls to the second box.
We can see that 8 of these 12 possible random distributions have the same number of distinct colors of balls in each box.
Probability is 8/12 = 0.66667
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> balls = [1,2,1,2]
<strong>Output:</strong> 0.60000
<strong>Explanation:</strong> The set of balls is [1, 2, 2, 3, 4, 4]. It is hard to display all the 180 possible random shuffles of this set but it is easy to check that 108 of them will have the same number of distinct colors in each box.
Probability = 108 / 180 = 0.6
</pre>

#### Constraints:
* `1 <= balls.length <= 8`
* `1 <= balls[i] <= 6`
* `sum(balls)` is even.

## Solutions (Python)

### 1. Solution
```Python
from math import comb


class Solution:
    def getProbability(self, balls: List[int]) -> float:
        n = sum(balls) // 2
        k = len(balls)
        used = 0
        dp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
              for _ in range(n * 2 + 1)]
        dp[0][0][k] = 1

        for i in range(k):
            tmp = [[[0] * (k * 2 + 1) for _ in range(n * 2 + 1)]
                   for _ in range(n * 2 + 1)]

            for leftballs, rightballs in zip(range(used + 1), range(used, -1, -1)):
                for offsetdiff in range(len(dp[0][0])):
                    if rightballs + balls[i] < len(dp) and offsetdiff > 0:
                        tmp[leftballs][rightballs + balls[i]][offsetdiff -
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(rightballs + balls[i], balls[i])
                    for toleft, toright in zip(range(1, balls[i]), range(balls[i] - 1, 0, -1)):
                        if leftballs + toleft < len(dp) and rightballs + toright < len(dp):
                            tmp[leftballs + toleft][rightballs + toright][offsetdiff] += dp[leftballs][rightballs][offsetdiff] * comb(
                                leftballs + toleft, toleft) * comb(rightballs + toright, toright)
                    if leftballs + balls[i] < len(dp) and offsetdiff + 1 < len(dp[0][0]):
                        tmp[leftballs + balls[i]][rightballs][offsetdiff +
                                                              1] += dp[leftballs][rightballs][offsetdiff] * comb(leftballs + balls[i], balls[i])

            used += balls[i]
            dp = tmp

        return dp[n][n][k] / sum(dp[n][n])
```
