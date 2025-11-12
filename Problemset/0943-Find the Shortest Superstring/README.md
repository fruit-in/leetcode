# 943. Find the Shortest Superstring
Given an array of strings `words`, return *the smallest string that contains each string in* `words` *as a substring*. If there are multiple valid strings of the smallest length, return **any of them**.

You may assume that no string in `words` is a substring of another string in `words`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["alex","loves","leetcode"]
<strong>Output:</strong> "alexlovesleetcode"
<strong>Explanation:</strong> All permutations of "alex","loves","leetcode" would also be accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["catg","ctaagt","gcta","ttca","atgcatc"]
<strong>Output:</strong> "gctaagttcatgcatc"
</pre>

#### Constraints:
* `1 <= words.length <= 12`
* `1 <= words[i].length <= 20`
* `words[i]` consists of lowercase English letters.
* All the strings of `words` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def shortestSuperstring(self, words: List[str]) -> str:
        @cache
        def shortestLength(mask: int, prev: int) -> int:
            if mask == (1 << n) - 1:
                return len(words[prev])

            ret = inf

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    ret = min(ret, len(words[prev]) + length)

            return ret

        n = len(words)
        overlap = {(n, i): 0 for i in range(n)}
        mask = 0
        prev = n
        parts = []

        for i in range(n):
            for j in range(i + 1, n):
                overlap[(i, j)] = 0
                overlap[(j, i)] = 0
                for k in range(1, min(len(words[i]), len(words[j]))):
                    if words[i][-k:] == words[j][:k]:
                        overlap[(i, j)] = k
                    if words[j][-k:] == words[i][:k]:
                        overlap[(j, i)] = k

        for _ in range(n):
            minlength = inf
            minlengthi = n

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    if length < minlength:
                        minlength = length
                        minlengthi = i

            parts.append(words[minlengthi][overlap[(prev, minlengthi)]:])
            mask |= 1 << minlengthi
            prev = minlengthi

        return ''.join(parts)
```
