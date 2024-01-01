# 1657. Determine if Two Strings Are Close
Two strings are considered **close** if you can attain one from the other using the following operations:

* Operation 1: Swap any two **existing** characters.
    * For example, `abcde -> aecdb`
* Operation 2: Transform **every** occurrence of one **existing** character into another **existing** character, and do the same with the other character.
    * For example, `aacabb -> bbcbaa` (all `a`'s turn into `b`'s, and all `b`'s turn into `a`'s)

You can use the operations on either string as many times as necessary.

Given two strings, `word1` and `word2`, return `true` *if* `word1` *and* `word2` *are **close**, and* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "abc", word2 = "bca"
<strong>Output:</strong> true
<strong>Explanation:</strong> You can attain word2 from word1 in 2 operations.
Apply Operation 1: "abc" -> "acb"
Apply Operation 1: "acb" -> "bca"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "a", word2 = "aa"
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to attain word2 from word1, or vice versa, in any number of operations.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word1 = "cabbba", word2 = "abbccc"
<strong>Output:</strong> true
<strong>Explanation:</strong> You can attain word2 from word1 in 3 operations.
Apply Operation 1: "cabbba" -> "caabbb"
Apply Operation 2: "caabbb" -> "baaccc"
Apply Operation 2: "baaccc" -> "abbccc"
</pre>

#### Constraints:
* <code>1 <= word1.length, word2.length <= 10<sup>5</sup></code>
* `word1` and `word2` contain only lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        if len(word1) != len(word2) or set(word1) != set(word2):
            return False

        count1 = [0] * 26
        count2 = [0] * 26

        for ch1, ch2 in zip(word1, word2):
            count1[ord(ch1) - 97] += 1
            count2[ord(ch2) - 97] += 1

        return sorted(count1) == sorted(count2)
```
