# 839. Similar String Groups
Two strings, `X` and `Y`, are considered similar if either they are identical or we can make them equivalent by swapping at most two letters (in distinct positions) within the string `X`.

For example, `"tars"` and `"rats"` are similar (swapping at positions `0` and `2`), and `"rats"` and `"arts"` are similar, but `"star"` is not similar to `"tars"`, `"rats"`, or `"arts"`.

Together, these form two connected groups by similarity: `{"tars", "rats", "arts"}` and `{"star"}`.  Notice that `"tars"` and `"arts"` are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.

We are given a list `strs` of strings where every string in `strs` is an anagram of every other string in `strs`. How many groups are there?

#### Example 1:
<pre>
<strong>Input:</strong> strs = ["tars","rats","arts","star"]
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> strs = ["omv","ovm"]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= strs.length <= 300`
* `1 <= strs[i].length <= 300`
* `strs[i]` consists of lowercase letters only.
* All words in `strs` have the same length and are anagrams of each other.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numSimilarGroups(self, strs: List[str]) -> int:
        m = len(strs)
        n = len(strs[0])
        parent = list(range(m))

        for i in range(m):
            for j in range(i + 1, m):
                count = 0

                for k in range(n):
                    if strs[i][k] != strs[j][k]:
                        count += 1

                if count <= 2:
                    while parent[i] != parent[parent[i]]:
                        parent[i] = parent[parent[i]]
                    while parent[j] != parent[parent[j]]:
                        parent[j] = parent[parent[j]]
                    if parent[i] < parent[j]:
                        parent[parent[j]] = parent[i]
                    else:
                        parent[parent[i]] = parent[j]

        for i in range(m):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

        return sum(1 for i in range(m) if parent[i] == i)
```
