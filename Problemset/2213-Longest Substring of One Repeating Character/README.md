# 2213. Longest Substring of One Repeating Character
You are given a **0-indexed** string `s`. You are also given a **0-indexed** string `queryCharacters` of length `k` and a **0-indexed** array of integer **indices** `queryIndices` of length `k`, both of which are used to describe `k` queries.

The <code>i<sup>th</sup></code> query updates the character in `s` at index `queryIndices[i]` to the character `queryCharacters[i]`.

Return *an array* `lengths` *of length* `k` *where* `lengths[i]` *is the **length** of the **longest substring** of* `s` *consisting of **only one repeating** character **after** the* <code>i<sup>th</sup></code> *query is performed*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "babacc", queryCharacters = "bcb", queryIndices = [1,3,3]
<strong>Output:</strong> [3,3,4]
<strong>Explanation:</strong>
- 1st query updates s = "bbbacc". The longest substring consisting of one repeating character is "bbb" with length 3.
- 2nd query updates s = "bbbccc".
  The longest substring consisting of one repeating character can be "bbb" or "ccc" with length 3.
- 3rd query updates s = "bbbbcc". The longest substring consisting of one repeating character is "bbbb" with length 4.
Thus, we return [3,3,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abyzz", queryCharacters = "aa", queryIndices = [2,1]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
- 1st query updates s = "abazz". The longest substring consisting of one repeating character is "zz" with length 2.
- 2nd query updates s = "aaazz". The longest substring consisting of one repeating character is "aaa" with length 3.
Thus, we return [2,3].
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters.
* `k == queryCharacters.length == queryIndices.length`
* <code>1 <= k <= 10<sup>5</sup></code>
* `queryCharacters` consists of lowercase English letters.
* `0 <= queryIndices[i] < s.length`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def longestRepeating(self, s: str, queryCharacters: str, queryIndices: List[int]) -> List[int]:
        size = 1 << ceil(log2(len(s)))
        treelength = [0] * (2 * size)
        treeleft = [('', 0)] * (2 * size)
        treeright = [('', 0)] * (2 * size)
        treerepeat = [0] * (2 * size)
        lengths = []

        for i, c in enumerate(s):
            treelength[size + i] = 1
            treeleft[size + i] = (c, 1)
            treeright[size + i] = (c, 1)
            treerepeat[size + i] = 1
        for i in range(size - 1, 0, -1):
            treelength[i] = treelength[2 * i] + treelength[2 * i + 1]
            treeleft[i] = treeleft[2 * i]
            treeright[i] = treeright[2 * i + 1]
            treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
            if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                treeleft[i] = (treeleft[i][0], treeleft[i]
                               [1] + treeleft[2 * i + 1][1])
            if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                treeright[i] = (treeright[i][0], treeright[i]
                                [1] + treeright[2 * i][1])
            if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                treerepeat[i] = max(
                    treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

        for c, i in zip(queryCharacters, queryIndices):
            i += size
            treeleft[i] = (c, 1)
            treeright[i] = (c, 1)
            while i > 1:
                i >>= 1
                treeleft[i] = treeleft[2 * i]
                treeright[i] = treeright[2 * i + 1]
                treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
                if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                    treeleft[i] = (treeleft[i][0], treeleft[i]
                                   [1] + treeleft[2 * i + 1][1])
                if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                    treeright[i] = (treeright[i][0], treeright[i]
                                    [1] + treeright[2 * i][1])
                if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                    treerepeat[i] = max(
                        treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

            lengths.append(treerepeat[1])

        return lengths
```
