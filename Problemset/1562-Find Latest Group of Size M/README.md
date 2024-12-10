# 1562. Find Latest Group of Size M
Given an array `arr` that represents a permutation of numbers from `1` to `n`.

You have a binary string of size `n` that initially has all its bits set to zero. At each step `i` (assuming both the binary string and `arr` are 1-indexed) from `1` to `n`, the bit at position `arr[i]` is set to `1`.

You are also given an integer `m`. Find the latest step at which there exists a group of ones of length `m`. A group of ones is a contiguous substring of `1`'s such that it cannot be extended in either direction.

Return *the latest step at which there exists a group of ones of length **exactly*** `m`. *If no such group exists, return* `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,5,1,2,4], m = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong>
Step 1: "00100", groups: ["1"]
Step 2: "00101", groups: ["1", "1"]
Step 3: "10101", groups: ["1", "1", "1"]
Step 4: "11101", groups: ["111", "1"]
Step 5: "11111", groups: ["11111"]
The latest step at which there exists a group of size 1 is step 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [3,1,5,4,2], m = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong>
Step 1: "00100", groups: ["1"]
Step 2: "10100", groups: ["1", "1"]
Step 3: "10101", groups: ["1", "1", "1"]
Step 4: "10111", groups: ["1", "111"]
Step 5: "11111", groups: ["11111"]
No group of size 2 exists during any step.
</pre>

#### Constraints:
* `n == arr.length`
* <code>1 <= m <= n <= 10<sup>5</sup></code>
* `1 <= arr[i] <= n`
* All integers in `arr` are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class Solution:
    def findLatestStep(self, arr: List[int], m: int) -> int:
        if len(arr) == m:
            return m

        n = len(arr)
        groups = SortedList([(1, n + 1)])

        for step in range(n - 1, -1, -1):
            i = groups.bisect_left((arr[step], n + 2)) - 1
            left = (groups[i][0], arr[step])
            right = (arr[step] + 1, groups[i][1])
            groups.pop(i)
            if left[1] - left[0] == m or right[1] - right[0] == m:
                return step
            if left[0] < left[1]:
                groups.add(left)
            if right[0] < right[1]:
                groups.add(right)

        return -1
```
