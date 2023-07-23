# 77. Combinations
Given two integers *n* and *k*, return all possible combinations of *k* numbers out of 1 ... *n*.

You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4, k = 2
<strong>Output:</strong> [
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1, k = 1
<strong>Output:</strong> [[1]]
</pre>

#### Constraints:
* `1 <= n <= 20`
* `1 <= k <= n`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        def foo(s: int, n: int, k: int) -> List[List[int]]:
            if k == 1:
                return [[i] for i in range(s, n + 1)]

            ret = []

            for i in range(s, n - k + 2):
                combs = foo(i + 1, n, k - 1)
                for comb in combs:
                    comb.append(i)
                ret.extend(combs)

            return ret

        return foo(1, n, k)
```
