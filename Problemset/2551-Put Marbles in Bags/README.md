# 2551. Put Marbles in Bags
You have `k` bags. You are given a **0-indexed** integer array `weights` where `weights[i]` is the weight of the <code>i<sup>th</sup></code> marble. You are also given the integer `k`.

Divide the marbles into the `k` bags according to the following rules:
* No bag is empty.
* If the <code>i<sup>th</sup></code> marble and <code>j<sup>th</sup></code> marble are in a bag, then all marbles with an index between the <code>i<sup>th</sup></code> and <code>j<sup>th</sup></code> indices should also be in that same bag.
* If a bag consists of all the marbles with an index from `i` to `j` inclusively, then the cost of the bag is `weights[i] + weights[j]`.

The **score** after distributing the marbles is the sum of the costs of all the `k` bags.

Return *the **difference** between the **maximum** and **minimum** scores among marble distributions*.

#### Example 1:
<pre>
<strong>Input:</strong> weights = [1,3,5,1], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The distribution [1],[3,5,1] results in the minimal score of (1+1) + (3+1) = 6.
The distribution [1,3],[5,1], results in the maximal score of (1+3) + (5+1) = 10.
Thus, we return their difference 10 - 6 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> weights = [1, 3], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> The only distribution possible is [1],[3].
Since both the maximal and minimal score are the same, we return 0.
</pre>

#### Constraints:
* <code>1 <= k <= weights.length <= 10<sup>5</sup></code>
* <code>1 <= weights[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def putMarbles(self, weights: List[int], k: int) -> int:
        weightsij = sorted(weights[i] + weights[i + 1]
                           for i in range(len(weights) - 1))

        return sum(weightsij[len(weights) - k:]) - sum(weightsij[:k - 1])
```
