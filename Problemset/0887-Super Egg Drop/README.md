# 887. Super Egg Drop
You are given `k` identical eggs and you have access to a building with `n` floors labeled from `1` to `n`.

You know that there exists a floor `f` where `0 <= f <= n` such that any egg dropped at a floor **higher** than `f` will **break**, and any egg dropped **at or below** floor `f` will **not break**.

Each move, you may take an unbroken egg and drop it from any floor `x` (where `1 <= x <= n`). If the egg breaks, you can no longer use it. However, if the egg does not break, you may **reuse** it in future moves.

Return *the **minimum number of moves** that you need to determine **with certainty** what the value of* `f` is.

#### Example 1:
<pre>
<strong>Input:</strong> k = 1, n = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Drop the egg from floor 1. If it breaks, we know that f = 0.
Otherwise, drop the egg from floor 2. If it breaks, we know that f = 1.
If it does not break, then we know f = 2.
Hence, we need at minimum 2 moves to determine with certainty what the value of f is.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 2, n = 6
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 3, n = 14
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= k <= 100`
* <code>1 <= n <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    @cache
    def superEggDrop(self, k: int, n: int) -> int:
        if n == 0:
            return 0
        if k == 1:
            return n

        x = bisect_right(range(1, n + 1), 0, key=lambda x: self.superEggDrop(k -
                         1, x - 1) - self.superEggDrop(k, n - x)) + 1
        ret = 1 + self.superEggDrop(k - 1, x - 1)
        if x > 0:
            ret = min(ret, 1 + self.superEggDrop(k, n - x + 1))

        return ret
```
