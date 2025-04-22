# 2111. Minimum Operations to Make the Array K-Increasing
You are given a **0-indexed** array `arr` consisting of `n` positive integers, and a positive integer `k`.

The array `arr` is called **K-increasing** if `arr[i-k] <= arr[i]` holds for every index `i`, where `k <= i <= n-1`.

* For example, `arr = [4, 1, 5, 2, 6, 2]` is K-increasing for `k = 2` because:
    * `arr[0] <= arr[2] (4 <= 5)`
    * `arr[1] <= arr[3] (1 <= 2)`
    * `arr[2] <= arr[4] (5 <= 6)`
    * `arr[3] <= arr[5] (2 <= 2)`
* However, the same `arr` is not K-increasing for `k = 1` (because `arr[0] > arr[1]`) or `k = 3` (because `arr[0] > arr[3]`).

In one **operation**, you can choose an index `i` and **change** `arr[i]` into **any** positive integer.

Return *the **minimum number of operations** required to make the array K-increasing for the given* `k`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [5,4,3,2,1], k = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong>
For k = 1, the resultant array has to be non-decreasing.
Some of the K-increasing arrays that can be formed are [5,6,7,8,9], [1,1,1,1,1], [2,2,3,4,4]. All of them require 4 operations.
It is suboptimal to change the array to, for example, [6,7,8,9,10] because it would take 5 operations.
It can be shown that we cannot make the array K-increasing in less than 4 operations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [4,1,5,2,6,2], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong>
This is the same example as the one in the problem description.
Here, for every index i where 2 <= i <= 5, arr[i-2] <= arr[i].
Since the given array is already K-increasing, we do not need to perform any operations.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [4,1,5,2,6,2], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Indices 3 and 5 are the only ones not satisfying arr[i-3] <= arr[i] for 3 <= i <= 5.
One of the ways we can make the array K-increasing is by changing arr[3] to 4 and arr[5] to 5.
The array will now be [4,1,5,4,6,5].
Note that there can be other ways to make the array K-increasing, but none of them require less than 2 operations.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* `1 <= arr[i], k <= arr.length`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def kIncreasing(self, arr: List[int], k: int) -> int:
        groups = [[] for _ in range(k)]
        ret = 0

        for i in range(len(arr)):
            groups[i % k].append(arr[i])

        for group in groups:
            lis = [0]

            for x in group:
                i = bisect.bisect(lis, x)
                if i == len(lis):
                    lis.append(x)
                else:
                    lis[i] = x

            ret += len(group) - len(lis) + 1

        return ret
```
