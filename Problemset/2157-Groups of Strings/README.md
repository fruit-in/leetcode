# 2157. Groups of Strings
You are given a **0-indexed** array of strings `words`. Each string consists of **lowercase English letters** only. No letter occurs more than once in any string of `words`.

Two strings `s1` and `s2` are said to be **connected** if the set of letters of `s2` can be obtained from the set of letters of `s1` by any **one** of the following operations:
* Adding exactly one letter to the set of the letters of `s1`.
* Deleting exactly one letter from the set of the letters of `s1`.
* Replacing exactly one letter from the set of the letters of `s1` with any letter, **including** itself.

The array `words` can be divided into one or more non-intersecting **groups**. A string belongs to a group if any **one** of the following is true:
* It is connected to **at least one** other string of the group.
* It is the **only** string present in the group.

Note that the strings in `words` should be grouped in such a manner that a string belonging to a group cannot be connected to a string present in any other group. It can be proved that such an arrangement is always unique.

Return *an array* `ans` *of size* `2` *where*:
* `ans[0]` *is the **maximum number** of groups* `words` *can be divided into, and*
* `ans[1]` *is the **size of the largest** group*.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["a","b","ab","cde"]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
- words[0] can be used to obtain words[1] (by replacing 'a' with 'b'), and words[2] (by adding 'b'). So words[0] is connected to words[1] and words[2].
- words[1] can be used to obtain words[0] (by replacing 'b' with 'a'), and words[2] (by adding 'a'). So words[1] is connected to words[0] and words[2].
- words[2] can be used to obtain words[0] (by deleting 'b'), and words[1] (by deleting 'a'). So words[2] is connected to words[0] and words[1].
- words[3] is not connected to any string in words.
Thus, words can be divided into 2 groups ["a","b","ab"] and ["cde"]. The size of the largest group is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["a","ab","abc"]
<strong>Output:</strong> [1,3]
<strong>Explanation:</strong>
- words[0] is connected to words[1].
- words[1] is connected to words[0] and words[2].
- words[2] is connected to words[1].
Since all strings are connected to each other, they should be grouped together.
Thus, the size of the largest group is 3.
</pre>

#### Constraints:
* <code>1 <= words.length <= 2 * 10<sup>4</sup></code>
* `1 <= words[i].length <= 26`
* `words[i]` consists of lowercase English letters only.
* No letter occurs more than once in `words[i]`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def groupStrings(self, words: List[str]) -> List[int]:
        counter = {}
        parent = {}
        groups = {}
        largestsize = 0

        for s1 in sorted(words, key=len):
            mask1 = reduce(lambda x, y: x | (1 << (ord(y) - 97)), s1, 0)
            if mask1 in counter:
                counter[mask1] += 1
                continue
            counter[mask1] = 1
            parent[mask1] = mask1

            for i in range(26):
                if (mask1 >> i) & 1 == 0:
                    continue

                mask2 = mask1 ^ (1 << i)
                if mask2 in counter:
                    while parent[mask2] != parent[parent[mask2]]:
                        parent[mask2] = parent[parent[mask2]]
                    parent[parent[mask2]] = mask1

                for j in range(26):
                    if (mask2 >> j) & 1 == 1:
                        continue

                    mask3 = mask2 ^ (1 << j)
                    if mask3 in counter:
                        while parent[mask3] != parent[parent[mask3]]:
                            parent[mask3] = parent[parent[mask3]]
                        parent[parent[mask3]] = mask1

        for mask, count in counter.items():
            while parent[mask] != parent[parent[mask]]:
                parent[mask] = parent[parent[mask]]
            groups[parent[mask]] = groups.get(parent[mask], 0) + count
            largestsize = max(largestsize, groups[parent[mask]])

        return [len(groups), largestsize]
```
