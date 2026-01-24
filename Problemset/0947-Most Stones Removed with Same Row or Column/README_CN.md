# 947. 移除最多的同行或同列石头
`n` 块石头放置在二维平面中的一些整数坐标点上。每个坐标点上最多只能有一块石头。

如果一块石头的 **同行或者同列** 上有其他石头存在，那么就可以移除这块石头。

给你一个长度为 `n` 的数组 `stones` ，其中 <code>stones[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 表示第 `i` 块石头的位置，返回 **可以移除的石子** 的最大数量。

#### 示例 1:
<pre>
<strong>输入:</strong> stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
<strong>输出:</strong> 5
<strong>解释:</strong> 一种移除 5 块石头的方法如下所示：
1. 移除石头 [2,2] ，因为它和 [2,1] 同行。
2. 移除石头 [2,1] ，因为它和 [0,1] 同列。
3. 移除石头 [1,2] ，因为它和 [1,0] 同行。
4. 移除石头 [1,0] ，因为它和 [0,0] 同列。
5. 移除石头 [0,1] ，因为它和 [0,0] 同行。
石头 [0,0] 不能移除，因为它没有与另一块石头同行/列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
<strong>输出:</strong> 3
<strong>解释:</strong> 一种移除 3 块石头的方法如下所示：
1. 移除石头 [2,2] ，因为它和 [2,0] 同行。
2. 移除石头 [2,0] ，因为它和 [0,0] 同列。
3. 移除石头 [0,2] ，因为它和 [0,0] 同行。
石头 [0,0] 和 [1,1] 不能移除，因为它们没有与另一块石头同行/列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> stones = [[0,0]]
<strong>输出:</strong> 0
<strong>解释:</strong> [0,0] 是平面上唯一一块石头，所以不可以移除它。
</pre>

#### 提示:
* `1 <= stones.length <= 1000`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup></code>
* 不会有两块石头放在同一个坐标点上

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        n = len(stones)
        parent = {(x, y): (x, y) for x, y in stones}
        group = set()
        stones.sort()

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if xi != xj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        stones.sort(key=lambda s: s[1])

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if yi != yj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        for x, y in stones:
            while parent[(x, y)] != parent[parent[(x, y)]]:
                parent[(x, y)] = parent[parent[(x, y)]]

            group.add(parent[(x, y)])

        return n - len(group)
```
