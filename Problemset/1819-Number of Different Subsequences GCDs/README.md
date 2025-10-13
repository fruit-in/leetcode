# 1819. Number of Different Subsequences GCDs
You are given an array `nums` that consists of positive integers.

The **GCD** of a sequence of numbers is defined as the greatest integer that divides **all** the numbers in the sequence evenly.

* For example, the GCD of the sequence `[4,6,16]` is `2`.

A **subsequence** of an array is a sequence that can be formed by removing some elements (possibly none) of the array.

* For example, `[2,5,10]` is a subsequence of `[1,2,1,2,4,1,5,10]`.

Return *the **number** of **different** GCDs among all **non-empty** subsequences of* `nums`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/17/image-1.png)
<pre>
<strong>Input:</strong> nums = [6,10,3]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The figure shows all the non-empty subsequences and their GCDs.
The different GCDs are 6, 10, 3, 2, and 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [5,15,40,5,6]
<strong>Output:</strong> 7
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 2 * 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countDifferentSubsequenceGCDs(self, nums: List[int]) -> int:
        isgcd = [False] * (max(nums) + 1)

        for i in nums:
            isgcd[i] = True

        for i in range(1, len(isgcd) // 2 + 1):
            mingcd = 0

            for j in range(i, len(isgcd), i):
                if isgcd[j]:
                    mingcd = gcd(mingcd, j) if mingcd > 0 else j
                if mingcd == i:
                    isgcd[i] = True
                    break

        return sum(isgcd)
```
