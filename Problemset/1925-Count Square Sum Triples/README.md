# 1925. Count Square Sum Triples
A **square triple** `(a,b,c)` is a triple where `a`, `b`, and `c` are **integers** and <code>a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup></code>.

Given an integer `n`, return *the number of **square triples** such that* `1 <= a, b, c <= n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> The square triples are (3,4,5) and (4,3,5).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 4
<strong>Explanation:</strong> The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).
</pre>

#### Constraints:
* `1 <= n <= 250`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countTriples(self, n: int) -> int:
        ret = 0

        for a in range(1, n):
            for b in range(1, n):
                c = int((a * a + b * b) ** 0.5)
                if c <= n and c * c == a * a + b * b:
                    ret += 1

        return ret
```
