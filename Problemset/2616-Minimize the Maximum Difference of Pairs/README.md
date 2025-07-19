# 2616. Minimize the Maximum Difference of Pairs
You are given a **0-indexed** integer array `nums` and an integer `p`. Find `p` pairs of indices of `nums` such that the **maximum** difference amongst all the pairs is **minimized**. Also, ensure no index appears more than once amongst the `p` pairs.

Note that for a pair of elements at the index `i` and `j`, the difference of this pair is `|nums[i] - nums[j]|`, where `|x|` represents the **absolute value** of `x`.

Return *the **minimum maximum** difference among all* `p` *pairs*. We define the maximum of an empty set to be zero.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,1,2,7,1,3], p = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> The first pair is formed from the indices 1 and 4, and the second pair is formed from the indices 2 and 5.
The maximum difference is max(|nums[1] - nums[4]|, |nums[2] - nums[5]|) = max(0, 1) = 1. Therefore, we return 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,2,1,2], p = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> Let the indices 1 and 3 form a pair. The difference of that pair is |2 - 2| = 0, which is the minimum we can attain.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* `0 <= p <= (nums.length)/2`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimizeMax(self, nums: List[int], p: int) -> int:
        def countPairs(maxdiff: int) -> int:
            i = 0
            ret = 0

            while i < len(diffs):
                if diffs[i] <= maxdiff:
                    ret += 1
                    i += 1
                i += 1

            return ret

        nums.sort()
        diffs = [nums[i + 1] - nums[i] for i in range(len(nums) - 1)]

        return bisect_left(range(0, max(diffs, default=0) + 1), p, key=countPairs)
```
