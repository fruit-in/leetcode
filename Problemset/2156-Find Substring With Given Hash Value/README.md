# 2156. Find Substring With Given Hash Value
The hash of a **0-indexed** string `s` of length `k`, given integers `p` and `m`, is computed using the following function:
* <code>hash(s, p, m) = (val(s[0]) * p<sup>0</sup> + val(s[1]) * p<sup>1</sup> + ... + val(s[k-1]) * p<sup>k-1</sup>) mod m</code>.

Where `val(s[i])` represents the index of `s[i]` in the alphabet from `val('a') = 1` to `val('z') = 26`.

You are given a string `s` and the integers `power`, `modulo`, `k`, and `hashValue`. Return `sub`, *the **first substring** of* `s` *of length* `k` *such that* `hash(sub, power, modulo) == hashValue`.

The test cases will be generated such that an answer always **exists**.

A **substring** is a contiguous non-empty sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leetcode", power = 7, modulo = 20, k = 2, hashValue = 0
<strong>Output:</strong> "ee"
<strong>Explanation:</strong> The hash of "ee" can be computed to be hash("ee", 7, 20) = (5 * 1 + 5 * 7) mod 20 = 40 mod 20 = 0.
"ee" is the first substring of length 2 with hashValue 0. Hence, we return "ee".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "fbxzaad", power = 31, modulo = 100, k = 3, hashValue = 32
<strong>Output:</strong> "fbx"
<strong>Explanation:</strong> The hash of "fbx" can be computed to be hash("fbx", 31, 100) = (6 * 1 + 2 * 31 + 24 * 312) mod 100 = 23132 mod 100 = 32.
The hash of "bxz" can be computed to be hash("bxz", 31, 100) = (2 * 1 + 24 * 31 + 26 * 312) mod 100 = 25732 mod 100 = 32.
"fbx" is the first substring of length 3 with hashValue 32. Hence, we return "fbx".
Note that "bxz" also has a hash of 32 but it appears later than "fbx".
</pre>

#### Constraints:
* <code>1 <= k <= s.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= power, modulo <= 10<sup>9</sup></code>
* `0 <= hashValue < modulo`
* `s` consists of lowercase English letters only.
* The test cases are generated such that an answer always **exists**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def subStrHash(self, s: str, power: int, modulo: int, k: int, hashValue: int) -> str:
        def val(c): return ord(c) - 96
        value = 0
        start = 0

        for i in range(len(s) - 1, -1, -1):
            value = (value * power + val(s[i])) % modulo
            if i + k < len(s):
                value = (value - val(s[i + k]) *
                         pow(power, k, modulo)) % modulo
            if value == hashValue:
                start = i

        return s[start:start + k]
```
