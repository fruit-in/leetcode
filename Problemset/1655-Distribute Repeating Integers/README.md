# 1655. Distribute Repeating Integers
You are given an array of `n` integers, `nums`, where there are at most `50` unique values in the array. You are also given an array of `m` customer order quantities, `quantity`, where `quantity[i]` is the amount of integers the <code>i<sup>th</sup></code> customer ordered. Determine if it is possible to distribute `nums` such that:
* The <code>i<sup>th</sup></code> customer gets **exactly** `quantity[i]` integers,
* The integers the <code>i<sup>th</sup></code> customer gets are **all equal**, and
* Every customer is satisfied.

Return `true` *if it is possible to distribute* `nums` *according to the above conditions*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], quantity = [2]
<strong>Output:</strong> false
<strong>Explanation:</strong> The 0th customer cannot be given two different integers.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,3], quantity = [2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The 0th customer is given [3,3]. The integers [1,2] are not used.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1,2,2], quantity = [2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> The 0th customer is given [1,1], and the 1st customer is given [2,2].
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 1000`
* `m == quantity.length`
* `1 <= m <= 10`
* <code>1 <= quantity[i] <= 10<sup>5</sup></code>
* There are at most `50` unique values in `nums`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def canDistribute(self, nums: List[int], quantity: List[int]) -> bool:
        def dfs(i: int) -> bool:
            if i == len(quantity):
                return True

            for j in range(len(count)):
                if count[j] >= quantity[i] and (j == 0 or count[j] != count[j - 1]):
                    count[j] -= quantity[i]
                    if dfs(i + 1):
                        return True
                    count[j] += quantity[i]

            return False

        count = sorted(collections.Counter(nums).values())[-len(quantity):]

        if count[-1] < max(quantity) or sum(count) < sum(quantity):
            return False

        return dfs(0)
```
