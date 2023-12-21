# 1061. Lexicographically Smallest Equivalent String
You are given two strings of the same length `s1` and `s2` and a string `baseStr`.

We say `s1[i]` and `s2[i]` are equivalent characters.

* For example, if `s1 = "abc"` and `s2 = "cde"`, then we have `'a' == 'c'`, `'b' == 'd'`, and `'c' == 'e'`.

Equivalent characters follow the usual rules of any equivalence relation:

* **Reflexivity:** `'a' == 'a'`.
* **Symmetry:** `'a' == 'b'` implies `'b' == 'a'`.
* **Transitivity:** `'a' == 'b'` and `'b' == 'c'` implies `'a' == 'c'`.

For example, given the equivalency information from `s1 = "abc"` and `s2 = "cde"`, `"acd"` and `"aab"` are equivalent strings of `baseStr = "eed"`, and `"aab"` is the lexicographically smallest equivalent string of `baseStr`.

Return *the lexicographically smallest equivalent string of* `baseStr` *by using the equivalency information from* `s1` *and* `s2`.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "parker", s2 = "morris", baseStr = "parser"
<strong>Output:</strong> "makkek"
<strong>Explanation:</strong> Based on the equivalency information in s1 and s2, we can group their characters as [m,p], [a,o], [k,r,s], [e,i].
The characters in each group are equivalent and sorted in lexicographical order.
So the answer is "makkek".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "hello", s2 = "world", baseStr = "hold"
<strong>Output:</strong> "hdld"
<strong>Explanation:</strong> Based on the equivalency information in s1 and s2, we can group their characters as [h,w], [d,e,o], [l,r].
So only the second letter 'o' in baseStr is changed to 'd', the answer is "hdld".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s1 = "leetcode", s2 = "programs", baseStr = "sourcecode"
<strong>Output:</strong> "aauaaaaada"
<strong>Explanation:</strong> We group the equivalent characters in s1 and s2 as [a,o,e,r,s,c], [l,p], [g,t] and [d,m], thus all letters in baseStr except 'u' and 'd' are transformed to 'a', the answer is "aauaaaaada".
</pre>

#### Constraints:
* `1 <= s1.length, s2.length, baseStr <= 1000`
* `s1.length == s2.length`
* `s1`, `s2`, and `baseStr` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def smallestEquivalentString(self, s1: str, s2: str, baseStr: str) -> str:
        equivalent = list(range(26))

        for ch1, ch2 in zip(s1, s2):
            ch1, ch2 = ord(ch1) - 97, ord(ch2) - 97

            while equivalent[equivalent[ch1]] != equivalent[ch1]:
                equivalent[ch1] = equivalent[equivalent[ch1]]
            while equivalent[equivalent[ch2]] != equivalent[ch2]:
                equivalent[ch2] = equivalent[equivalent[ch2]]

            if equivalent[ch1] < equivalent[ch2]:
                equivalent[equivalent[ch2]] = equivalent[ch1]
            else:
                equivalent[equivalent[ch1]] = equivalent[ch2]

        for ch in range(26):
            while equivalent[equivalent[ch]] != equivalent[ch]:
                equivalent[ch] = equivalent[equivalent[ch]]

        return ''.join(chr(equivalent[ord(ch) - 97] + 97) for ch in baseStr)
```
