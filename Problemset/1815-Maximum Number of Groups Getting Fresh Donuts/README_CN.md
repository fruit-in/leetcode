# 1815. 得到新鲜甜甜圈的最多组数
有一个甜甜圈商店，每批次都烤 `batchSize` 个甜甜圈。这个店铺有个规则，就是在烤一批新的甜甜圈时，之前 **所有** 甜甜圈都必须已经全部销售完毕。给你一个整数 `batchSize` 和一个整数数组 `groups` ，数组中的每个整数都代表一批前来购买甜甜圈的顾客，其中 `groups[i]` 表示这一批顾客的人数。每一位顾客都恰好只要一个甜甜圈。

当有一批顾客来到商店时，他们所有人都必须在下一批顾客来之前购买完甜甜圈。如果一批顾客中第一位顾客得到的甜甜圈不是上一组剩下的，那么这一组人都会很开心。

你可以随意安排每批顾客到来的顺序。请你返回在此前提下，**最多** 有多少组人会感到开心。

#### 示例 1:
<pre>
<strong>输入:</strong> batchSize = 3, groups = [1,2,3,4,5,6]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以将这些批次的顾客顺序安排为 [6,2,4,5,1,3] 。那么第 1，2，4，6 组都会感到开心。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> batchSize = 4, groups = [1,3,2,5,2,2,1,6]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= batchSize <= 9`
* `1 <= groups.length <= 30`
* <code>1 <= groups[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
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
