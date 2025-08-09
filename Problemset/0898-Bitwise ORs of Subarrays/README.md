# 898. Bitwise ORs of Subarrays
Given an integer array `arr`, return *the number of distinct bitwise ORs of all the non-empty subarrays of* `arr`.

The bitwise OR of a subarray is the bitwise OR of each integer in the subarray. The bitwise OR of a subarray of one integer is that integer.

A **subarray** is a contiguous non-empty sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one possible result: 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The possible subarrays are [1], [1], [2], [1, 1], [1, 2], [1, 1, 2].
These yield the results 1, 1, 2, 1, 3, 3.
There are 3 unique values, so the answer is 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The possible results are 1, 2, 3, 4, 6, and 7.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 5 * 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def subarrayBitwiseORs(self, arr: List[int]) -> int:
        subarrayors = set(arr)

        for i in range(len(arr)):
            for j in range(i - 1, -1, -1):
                if arr[j] | arr[i] == arr[j]:
                    break
                arr[j] |= arr[i]
                subarrayors.add(arr[j])

        return len(subarrayors)
```
