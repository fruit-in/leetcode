# 1718. Construct the Lexicographically Largest Valid Sequence
Given an integer `n`, find a sequence with elements in the range `[1, n]` that satisfies all of the following:
* The integer `1` occurs once in the sequence.
* Each integer between `2` and `n` occurs twice in the sequence.
* For every integer `i` between `2` and `n`, the **distance** between the two occurrences of `i` is exactly `i`.

The **distance** between two numbers on the sequence, `a[i]` and `a[j]`, is the absolute difference of their indices, `|j - i|`.

Return *the **lexicographically largest** sequence. It is guaranteed that under the given constraints, there is always a solution*.

A sequence `a` is lexicographically larger than a sequence `b` (of the same length) if in the first position where `a` and `b` differ, sequence `a` has a number greater than the corresponding number in `b`. For example, `[0,1,9,0]` is lexicographically larger than `[0,1,5,6]` because the first position they differ is at the third number, and `9` is greater than `5`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3
<strong>Output:</strong> [3,1,2,3,2]
<strong>Explanation:</strong> [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> [5,3,1,4,3,5,2,4,2]
</pre>

#### Constraints:
* `1 <= n <= 20`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def constructDistancedSequence(self, n: int) -> List[int]:
        def dfs(i: int, mask: int) -> bool:
            if i == len(a):
                return mask == (1 << (n + 1)) - 2
            if a[i] > 0:
                return dfs(i + 1, mask)

            for x in range(n, 0, -1):
                if (mask >> x) & 1 == 0 and (x == 1 or (i + x < len(a) and a[i + x] == 0)):
                    a[i] = x
                    if x > 1:
                        a[i + x] = x
                    if dfs(i + 1, mask | (1 << x)):
                        return True
                    a[i] = 0
                    if x > 1:
                        a[i + x] = 0

            return False

        a = [0] * (n * 2 - 1)
        dfs(0, 0)

        return a
```
