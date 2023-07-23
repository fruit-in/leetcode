# 2053. Kth Distinct String in an Array
A **distinct string** is a string that is present only **once** in an array.

Given an array of strings `arr`, and an integer `k`, return the <code>k<sup>th</sup></code> ***distinct string** present in* `arr`. If there are **fewer** than `k` distinct strings, return *an **empty string*** `""`.

Note that the strings are considered in the **order in which they appear** in the array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = ["d","b","c","b","c","a"], k = 2
<strong>Output:</strong> "a"
<strong>Explanation:</strong>
The only distinct strings in arr are "d" and "a".
"d" appears 1st, so it is the 1st distinct string.
"a" appears 2nd, so it is the 2nd distinct string.
Since k == 2, "a" is returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = ["aaa","aa","a"], k = 1
<strong>Output:</strong> "aaa"
<strong>Explanation:</strong>
All strings in arr are distinct, so the 1st string "aaa" is returned.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = ["a","b","a"], k = 3
<strong>Output:</strong> ""
<strong>Explanation:</strong>
The only distinct string is "b". Since there are fewer than 3 distinct strings, we return an empty string "".
</pre>

#### Constraints:
* `1 <= k <= arr.length <= 1000`
* `1 <= arr[i].length <= 5`
* `arr[i]` consists of lowercase English letters.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def kthDistinct(self, arr: List[str], k: int) -> str:
        count = Counter(arr)
        distincts = [s for s in arr if count[s] == 1]

        return distincts[k - 1] if len(distincts) >= k else ""
```
