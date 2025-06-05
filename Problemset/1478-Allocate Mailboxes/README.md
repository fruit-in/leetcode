# 1478. Allocate Mailboxes
Given the array `houses` where `houses[i]` is the location of the <code>i<sup>th</sup></code> house along a street and an integer `k`, allocate `k` mailboxes in the street.

Return *the **minimum** total distance between each house and its nearest mailbox*.

The test cases are generated so that the answer fits in a 32-bit integer.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png)
<pre>
<strong>Input:</strong> houses = [1,4,8,10,20], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> Allocate mailboxes in position 3, 9 and 20.
Minimum total distance from each houses to nearest mailboxes is |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png)
<pre>
<strong>Input:</strong> houses = [2,3,5,12,18], k = 2
<strong>Output:</strong> 9
<strong>Explanation:</strong> Allocate mailboxes in position 3 and 14.
Minimum total distance from each houses to nearest mailboxes is |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9.
</pre>

#### Constraints:
* `1 <= k <= houses.length <= 100`
* <code>1 <= houses[i] <= 10<sup>4</sup></code>
* All the integers of `houses` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minDistance(self, houses: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(houses):
                return 0
            if k == 0:
                return inf

            ret = inf

            for j in range(i + 1, len(houses) + 1):
                mid = (i + j) // 2
                distl = houses[mid] * (mid - i) - prefixsum[mid] + prefixsum[i]
                distr = prefixsum[j] - prefixsum[mid] - houses[mid] * (j - mid)
                ret = min(ret, distl + distr + dfs(j, k - 1))

            return ret

        prefixsum = [0] * (len(houses) + 1)
        houses.sort()

        for i in range(len(houses)):
            prefixsum[i + 1] = prefixsum[i] + houses[i]

        return dfs(0, k)
```
