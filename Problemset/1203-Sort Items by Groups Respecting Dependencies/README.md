# 1203. Sort Items by Groups Respecting Dependencies
There are `n` items each belonging to zero or one of `m` groups where `group[i]` is the group that the `i`-th item belongs to and it's equal to `-1` if the `i`-th item belongs to no group. The items and the groups are zero indexed. A group can have no item belonging to it.

Return a sorted list of the items such that:
* The items that belong to the same group are next to each other in the sorted list.
* There are some relations between these items where `beforeItems[i]` is a list containing all the items that should come before the `i`-th item in the sorted array (to the left of the `i`-th item).

Return any solution if there is more than one solution and return an **empty list** if there is no solution.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/11/1359_ex1.png)
<pre>
<strong>Input:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3,6],[],[],[]]
<strong>Output:</strong> [6,3,4,1,5,2,0,7]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3],[],[4],[]]
<strong>Output:</strong> []
<strong>Explanation:</strong> This is the same as example 1 except that 4 needs to be before 6 in the sorted list.
</pre>

#### Constraints:
* <code>1 <= m <= n <= 3 * 10<sup>4</sup></code>
* `group.length == beforeItems.length == n`
* `-1 <= group[i] <= m - 1`
* `0 <= beforeItems[i].length <= n - 1`
* `0 <= beforeItems[i][j] <= n - 1`
* `i != beforeItems[i][j]`
* `beforeItems[i]` does not contain duplicates elements.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sortItems(self, n: int, m: int, group: List[int], beforeitems: List[List[int]]) -> List[int]:
        groupitems = [[] for _ in range(m + 1)]
        afteritems = [[] for _ in range(n)]
        itemindegree = [0] * n
        groupindegree = [0] * m
        ret = []

        for i in range(len(group)):
            groupitems[group[i]].append(i)

            for j in beforeitems[i]:
                afteritems[j].append(i)
                itemindegree[i] += 1
                if group[i] != -1 and group[j] != group[i]:
                    groupindegree[group[i]] += 1

        nogroupstack = [i for i in groupitems[-1] if itemindegree[i] == 0]
        groupstack = [i for i in range(m) if groupindegree[i] == 0]

        while nogroupstack or groupstack:
            while nogroupstack:
                i = nogroupstack.pop()
                ret.append(i)

                for j in afteritems[i]:
                    itemindegree[j] -= 1
                    if itemindegree[j] == 0 and group[j] == -1:
                        nogroupstack.append(j)
                    if group[j] != -1:
                        groupindegree[group[j]] -= 1
                        if groupindegree[group[j]] == 0:
                            groupstack.append(group[j])

            while groupstack:
                g = groupstack.pop()
                itemstack = [i for i in groupitems[g] if itemindegree[i] == 0]
                temp = []

                while itemstack:
                    i = itemstack.pop()
                    temp.append(i)

                    for j in afteritems[i]:
                        itemindegree[j] -= 1
                        if itemindegree[j] == 0:
                            if group[j] == -1:
                                nogroupstack.append(j)
                            elif group[j] == g:
                                itemstack.append(j)
                        if group[j] != -1 and group[j] != g:
                            groupindegree[group[j]] -= 1
                            if groupindegree[group[j]] == 0:
                                groupstack.append(group[j])

                ret.extend(temp)

        return ret if len(ret) == n else []
```
