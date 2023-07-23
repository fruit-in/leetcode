# 1754. Largest Merge Of Two Strings
You are given two strings `word1` and `word2`. You want to construct a string `merge` in the following way: while either `word1` or `word2` are non-empty, choose **one** of the following options:
* If `word1` is non-empty, append the **first** character in `word1` to `merge` and delete it from `word1`.
    * For example, if `word1 = "abc"` and `merge = "dv"`, then after choosing this operation, `word1 = "bc"` and `merge = "dva"`.
* If `word2` is non-empty, append the **first** character in `word2` to `merge` and delete it from `word2`.
    * For example, if `word2 = "abc"` and `merge = ""`, then after choosing this operation, `word2 = "bc"` and `merge = "a"`.

Return *the lexicographically **largest*** `merge` *you can construct*.

A string `a` is lexicographically larger than a string `b` (of the same length) if in the first position where `a` and `b` differ, `a` has a character strictly larger than the corresponding character in `b`. For example, `"abcd"` is lexicographically larger than `"abcc"` because the first position they differ is at the fourth character, and `d` is greater than `c`.

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "cabaa", word2 = "bcaaa"
<strong>Output:</strong> "cbcabaaaaa"
<strong>Explanation:</strong> One way to get the lexicographically largest merge is:
- Take from word1: merge = "c", word1 = "abaa", word2 = "bcaaa"
- Take from word2: merge = "cb", word1 = "abaa", word2 = "caaa"
- Take from word2: merge = "cbc", word1 = "abaa", word2 = "aaa"
- Take from word1: merge = "cbca", word1 = "baa", word2 = "aaa"
- Take from word1: merge = "cbcab", word1 = "aa", word2 = "aaa"
- Append the remaining 5 a's from word1 and word2 at the end of merge.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "abcabc", word2 = "abdcaba"
<strong>Output:</strong> "abdcabcabcaba"
</pre>

#### Constraints:
* `1 <= word1.length, word2.length <= 3000`
* `word1` and `word2` consist only of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def largestMerge(self, word1: str, word2: str) -> str:
        i = 0
        j = 0
        merge = ""

        while i < len(word1) and j < len(word2):
            if word1[i:] > word2[j:]:
                merge += word1[i]
                i += 1
            else:
                merge += word2[j]
                j += 1

        return merge + word1[i:] + word2[j:]
```
