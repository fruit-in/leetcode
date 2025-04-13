# 2382. Maximum Segment Sum After Removals
You are given two **0-indexed** integer arrays `nums` and `removeQueries`, both of length `n`. For the <code>i<sup>th</sup></code> query, the element in `nums` at the index `removeQueries[i]` is removed, splitting `nums` into different segments.

A **segment** is a contiguous sequence of **positive** integers in `nums`. A **segment sum** is the sum of every element in a segment.

Return *an integer array* `answer`, *of length* `n`, *where* `answer[i]` *is the **maximum** segment sum after applying the* <code>i<sup>th</sup></code> *removal*.

**Note:** The same index will **not** be removed more than once.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,5,6,1], removeQueries = [0,3,2,4,1]
<strong>Output:</strong> [14,7,2,2,0]
<strong>Explanation:</strong> Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 0th element, nums becomes [0,2,5,6,1] and the maximum segment sum is 14 for segment [2,5,6,1].
Query 2: Remove the 3rd element, nums becomes [0,2,5,0,1] and the maximum segment sum is 7 for segment [2,5].
Query 3: Remove the 2nd element, nums becomes [0,2,0,0,1] and the maximum segment sum is 2 for segment [2].
Query 4: Remove the 4th element, nums becomes [0,2,0,0,0] and the maximum segment sum is 2 for segment [2].
Query 5: Remove the 1st element, nums becomes [0,0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [14,7,2,2,0].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,2,11,1], removeQueries = [3,2,1,0]
<strong>Output:</strong> [16,5,3,0]
<strong>Explanation:</strong> Using 0 to indicate a removed element, the answer is as follows:
Query 1: Remove the 3rd element, nums becomes [3,2,11,0] and the maximum segment sum is 16 for segment [3,2,11].
Query 2: Remove the 2nd element, nums becomes [3,2,0,0] and the maximum segment sum is 5 for segment [3,2].
Query 3: Remove the 1st element, nums becomes [3,0,0,0] and the maximum segment sum is 3 for segment [3].
Query 4: Remove the 0th element, nums becomes [0,0,0,0] and the maximum segment sum is 0, since there are no segments.
Finally, we return [16,5,3,0].
</pre>

#### Constraints:
* `n == nums.length == removeQueries.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* `0 <= removeQueries[i] < n`
* All the values of `removeQueries` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maximumSegmentSum(self, nums: List[int], removeQueries: List[int]) -> List[int]:
        from sortedcontainers import SortedList

        n = len(nums)
        prefixsum = [0] * (n + 1)
        segments = SortedList([(0, n)])
        segmentsums = SortedList([sum(nums)])
        answer = [0] * n

        for i in range(n):
            prefixsum[i + 1] = prefixsum[i] + nums[i]

        for i, j in enumerate(removeQueries[:-1]):
            k = segments.bisect_right((j, n)) - 1
            k, l = segments.pop(k)
            segmentsums.discard(prefixsum[l] - prefixsum[k])
            if j != k:
                segments.add((k, j))
                segmentsums.add(prefixsum[j] - prefixsum[k])
            if j != l - 1:
                segments.add((j + 1, l))
                segmentsums.add(prefixsum[l] - prefixsum[j + 1])

            answer[i] = segmentsums[-1]

        return answer
```
