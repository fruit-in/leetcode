# 1202. Smallest String With Swaps
You are given a string `s`, and an array of pairs of indices in the string `pairs` where `pairs[i] = [a, b]` indicates 2 indices(0-indexed) of the string.

You can swap the characters at any pair of indices in the given `pairs` **any number of times**.

Return the lexicographically smallest string that `s` can be changed to after using the swaps.

#### Example 1:
<pre>
<strong>Input:</strong> s = "dcab", pairs = [[0,3],[1,2]]
<strong>Output:</strong> "bacd"
<strong>Explanation:</strong>
Swap s[0] and s[3], s = "bcad"
Swap s[1] and s[2], s = "bacd"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "dcab", pairs = [[0,3],[1,2],[0,2]]
<strong>Output:</strong> "abcd"
<strong>Explanation:</strong>
Swap s[0] and s[3], s = "bcad"
Swap s[0] and s[2], s = "acbd"
Swap s[1] and s[2], s = "abcd"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "cba", pairs = [[0,1],[1,2]]
<strong>Output:</strong> "abc"
<strong>Explanation:</strong>
Swap s[0] and s[1], s = "bca"
Swap s[1] and s[2], s = "bac"
Swap s[0] and s[1], s = "abc"
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `0 <= pairs.length <= 10^5`
* `0 <= pairs[i][0], pairs[i][1] < s.length`
* `s` only contains lower case English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        parent = list(range(len(s)))
        groups = {}
        chars = []

        for a, b in pairs:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(len(s)):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = []
            groups[parent[i]].append(i)

        for group in groups:
            groups[group].sort(key=lambda i: -ord(s[i]))

        for i in range(len(s)):
            chars.append(s[groups[parent[i]].pop()])

        return ''.join(chars)
```
