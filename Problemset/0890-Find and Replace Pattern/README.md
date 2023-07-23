# 890. Find and Replace Pattern
Given a list of strings `words` and a string `pattern`, return *a list of* `words[i]` *that match* `pattern`. You may return the answer in **any order**.

A word matches the pattern if there exists a permutation of letters `p` so that after replacing every letter `x` in the pattern with `p(x)`, we get the desired word.

Recall that a permutation of letters is a bijection from letters to letters: every letter maps to another letter, and no two letters map to the same letter.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["abc","deq","mee","aqq","dkd","ccc"], pattern = "abb"
<strong>Output:</strong> ["mee","aqq"]
<strong>Explanation:</strong> "mee" matches the pattern because there is a permutation {a -> m, b -> e, ...}.
"ccc" does not match the pattern because {a -> c, b -> c, ...} is not a permutation, since a and b map to the same letter.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["a","b","c"], pattern = "a"
<strong>Output:</strong> ["a","b","c"]
</pre>

#### Constraints:
* `1 <= pattern.length <= 20`
* `1 <= words.length <= 50`
* `words[i].length == pattern.length`
* `pattern` and `words[i]` are lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        ret = []

        for word in words:
            map_wp = {}
            map_pw = {}

            for wl, pl in zip(word, pattern):
                if wl not in map_wp and pl not in map_pw:
                    map_wp[wl] = pl
                    map_pw[pl] = wl
                elif wl not in map_wp or pl not in map_pw:
                    break
                elif map_wp[wl] != pl or map_pw[pl] != wl:
                    break
            else:
                ret.append(word)

        return ret
```
