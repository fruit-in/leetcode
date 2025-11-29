# 1815. Maximum Number of Groups Getting Fresh Donuts
There is a donuts shop that bakes donuts in batches of `batchSize`. They have a rule where they must serve **all** of the donuts of a batch before serving any donuts of the next batch. You are given an integer `batchSize` and an integer array `groups`, where `groups[i]` denotes that there is a group of `groups[i]` customers that will visit the shop. Each customer will get exactly one donut.

When a group visits the shop, all customers of the group must be served before serving any of the following groups. A group will be happy if they all get fresh donuts. That is, the first customer of the group does not receive a donut that was left over from the previous group.

You can freely rearrange the ordering of the groups. Return *the **maximum** possible number of happy groups after rearranging the groups*.

#### Example 1:
<pre>
<strong>Input:</strong> batchSize = 3, groups = [1,2,3,4,5,6]
<strong>Output:</strong> 4
<strong>Explanation:</strong> You can arrange the groups as [6,2,4,5,1,3]. Then the 1st, 2nd, 4th, and 6th groups will be happy.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> batchSize = 4, groups = [1,3,2,5,2,2,1,6]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= batchSize <= 9`
* `1 <= groups.length <= 30`
* <code>1 <= groups[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxHappyGroups(self, batchSize: int, groups: List[int]) -> int:
        @cache
        def dfs(count: Tuple[int]) -> int:
            count = list(count)
            remain = (total - sum(i * count[i]
                      for i in range(1, batchSize))) % batchSize
            ret = 0

            for i in range(1, batchSize):
                if count[i] == 0:
                    continue

                count[i] -= 1
                if remain == 0:
                    ret = max(ret, 1 + dfs(tuple(count)))
                else:
                    ret = max(ret, dfs(tuple(count)))
                count[i] += 1

            return ret

        count = [0] * batchSize
        total = 0

        for x in groups:
            count[x % batchSize] += 1
            total += x % batchSize

        return count[0] + dfs(tuple(count))
```
