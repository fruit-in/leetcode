# 2081. Sum of k-Mirror Numbers
A **k-mirror number** is a **positive** integer **without leading zeros** that reads the same both forward and backward in base-10 **as well as** in base-k.

* For example, `9` is a 2-mirror number. The representation of `9` in base-10 and base-2 are `9` and `1001` respectively, which read the same both forward and backward.
* On the contrary, `4` is not a 2-mirror number. The representation of `4` in base-2 is `100`, which does not read the same both forward and backward.

Given the base `k` and the number `n`, return *the **sum** of the* `n` ***smallest** k-mirror numbers*.

#### Example 1:
<pre>
<strong>Input:</strong> k = 2, n = 5
<strong>Output:</strong> 25
<strong>Explanation:</strong>
The 5 smallest 2-mirror numbers and their representations in base-2 are listed as follows:
  base-10    base-2
    1          1
    3          11
    5          101
    7          111
    9          1001
Their sum = 1 + 3 + 5 + 7 + 9 = 25.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 3, n = 7
<strong>Output:</strong> 499
<strong>Explanation:</strong>
The 7 smallest 3-mirror numbers are and their representations in base-3 are listed as follows:
  base-10    base-3
    1          1
    2          2
    4          11
    8          22
    121        11111
    151        12121
    212        21212
Their sum = 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 7, n = 17
<strong>Output:</strong> 20379000
<strong>Explanation:</strong> The 17 smallest 7-mirror numbers are:
1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596
</pre>

#### Constraints:
* `2 <= k <= 9`
* `1 <= n <= 30`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def kMirror(self, k: int, n: int) -> int:
        x = 1
        nums = []

        while True:
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[-2::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if d == d[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)
            for a in range(x, 10 * x):
                b = c = int(str(a) + str(a)[::-1])
                d = ''
                while c > 0:
                    d = str(c % k) + d
                    c //= k
                if str(d) == str(d)[::-1]:
                    nums.append(b)
                if len(nums) == n:
                    return sum(nums)

            x *= 10
```
