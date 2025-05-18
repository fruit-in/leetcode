# 2411. Smallest Subarrays With Maximum Bitwise OR
You are given a **0-indexed** array `nums` of length `n`, consisting of non-negative integers. For each index `i` from `0` to `n - 1`, you must determine the size of the **minimum sized** non-empty subarray of nums starting at `i` (**inclusive**) that has the **maximum** possible **bitwise OR**.

* In other words, let <code>B<sup>ij</sup></code> be the bitwise OR of the subarray `nums[i...j]`. You need to find the smallest subarray starting at `i`, such that bitwise OR of this subarray is equal to <code>max(B<sup>ik</sup>)</code> where `i <= k <= n - 1`.

The bitwise OR of an array is the bitwise OR of all the numbers in it.

Return *an integer array* `answer` *of size* `n` *where* `answer[i]` *is the length of the **minimum** sized subarray starting at* `i` *with **maximum** bitwise OR*.

A **subarray** is a contiguous non-empty sequence of elements within an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,0,2,1,3]
<strong>Output:</strong> [3,3,2,2,1]
<strong>Explanation:</strong>
The maximum possible bitwise OR starting at any index is 3.
- Starting at index 0, the shortest subarray that yields it is [1,0,2].
- Starting at index 1, the shortest subarray that yields the maximum bitwise OR is [0,2,1].
- Starting at index 2, the shortest subarray that yields the maximum bitwise OR is [2,1].
- Starting at index 3, the shortest subarray that yields the maximum bitwise OR is [1,3].
- Starting at index 4, the shortest subarray that yields the maximum bitwise OR is [3].
Therefore, we return [3,3,2,2,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> [2,1]
<strong>Explanation:</strong>
Starting at index 0, the shortest subarray that yields the maximum bitwise OR is of length 2.
Starting at index 1, the shortest subarray that yields the maximum bitwise OR is of length 1.
Therefore, we return [2,1].
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def smallestSubarrays(self, nums: List[int]) -> List[int]:
        def subarrayOR(j: int) -> int:
            maxor = 0
            for k in range(m):
                if prefixcount[j + 1][k] - prefixcount[i][k] > 0:
                    maxor |= 1 << k

            return maxor

        m = int(math.log2(max(1, max(nums)))) + 1
        n = len(nums)
        prefixcount = [[0] * m]
        answer = [0] * n

        for x in nums:
            prefixcount.append(prefixcount[-1].copy())

            for i in range(m):
                prefixcount[-1][i] += (x >> i) & 1

        for i in range(n):
            maxor = 0
            for j in range(m):
                if prefixcount[n][j] - prefixcount[i][j] > 0:
                    maxor |= 1 << j

            j = bisect.bisect_left(range(n), maxor, lo=i, key=subarrayOR)
            answer[i] = j - i + 1

        return answer
```
