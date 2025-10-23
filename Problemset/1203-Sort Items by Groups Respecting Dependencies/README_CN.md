# 1203. 项目管理
有 `n` 个项目，每个项目或者不属于任何小组，或者属于 `m` 个小组之一。`group[i]` 表示第 `i` 个项目所属的小组，如果第 `i` 个项目不属于任何小组，则 `group[i]` 等于 `-1`。项目和小组都是从零开始编号的。可能存在小组不负责任何项目，即没有任何项目属于这个小组。

请你帮忙按要求安排这些项目的进度，并返回排序后的项目列表：
* 同一小组的项目，排序后在列表中彼此相邻。
* 项目之间存在一定的依赖关系，我们用一个列表 `beforeItems` 来表示，其中 `beforeItems[i]` 表示在进行第 `i` 个项目前（位于第 `i` 个项目左侧）应该完成的所有项目。

如果存在多个解决方案，只需要返回其中任意一个即可。如果没有合适的解决方案，就请返回一个 **空列表** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/09/11/1359_ex1.png)
<pre>
<strong>输入:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3,6],[],[],[]]
<strong>输出:</strong> [6,3,4,1,5,2,0,7]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 8, m = 2, group = [-1,-1,1,0,0,1,0,-1], beforeItems = [[],[6],[5],[6],[3],[],[4],[]]
<strong>输出:</strong> []
<strong>解释:</strong> 与示例 1 大致相同，但是在排序后的列表中，4 必须放在 6 的前面。
</pre>

#### 提示:
* <code>1 <= m <= n <= 3 * 10<sup>4</sup></code>
* `group.length == beforeItems.length == n`
* `-1 <= group[i] <= m - 1`
* `0 <= beforeItems[i].length <= n - 1`
* `0 <= beforeItems[i][j] <= n - 1`
* `i != beforeItems[i][j]`
* `beforeItems[i]` 不含重复元素

## 题解 (Python)

### 1. 题解
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
