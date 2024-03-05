# 765. 情侣牵手
`n` 对情侣坐在连续排列的 `2n` 个座位上，想要牵到对方的手。

人和座位由一个整数数组 `row` 表示，其中 `row[i]` 是坐在第 `i` 个座位上的人的 **ID**。情侣们按顺序编号，第一对是 `(0, 1)`，第二对是 `(2, 3)`，以此类推，最后一对是 `(2n-2, 2n-1)`。

返回 *最少交换座位的次数，以便每对情侣可以并肩坐在一起*。 *每次*交换可选择任意两人，让他们站起来交换座位。

#### 示例 1:
<pre>
<strong>输入:</strong> row = [0,2,1,3]
<strong>输出:</strong> 1
<strong>解释:</strong> 只需要交换row[1]和row[2]的位置即可。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> row = [3,2,0,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 无需交换座位，所有的情侣都已经可以手牵手了。
</pre>

#### 提示:
* `2n == row.length`
* `2 <= n <= 30`
* `n` 是偶数
* `0 <= row[i] < 2n`
* `row` 中所有元素均**无重复**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minSwapsCouples(self, row: List[int]) -> int:
        row = [person // 2 for person in row]
        parent = list(range(len(row) // 2))
        groups = {}

        for i in range(0, len(row), 2):
            if row[i] == row[i + 1]:
                continue

            while parent[parent[row[i]]] != parent[row[i]]:
                parent[row[i]] = parent[parent[row[i]]]
            while parent[parent[row[i + 1]]] != parent[row[i + 1]]:
                parent[row[i + 1]] = parent[parent[row[i + 1]]]

            if parent[row[i]] < parent[row[i + 1]]:
                parent[parent[row[i + 1]]] = parent[row[i]]
            else:
                parent[parent[row[i]]] = parent[row[i + 1]]

        for person in parent:
            while parent[parent[person]] != parent[person]:
                parent[person] = parent[parent[person]]

            groups[parent[person]] = groups.get(parent[person], 0) + 1

        return sum(x - 1 for x in groups.values())
```
