# 1916. 统计为蚁群构筑房间的不同顺序
你是一只蚂蚁，负责为蚁群构筑 `n` 间编号从 `0` 到 `n-1` 的新房间。给你一个 **下标从 0 开始** 且长度为 `n` 的整数数组 `prevRoom` 作为扩建计划。其中，`prevRoom[i]` 表示在构筑房间 `i` 之前，你必须先构筑房间 `prevRoom[i]` ，并且这两个房间必须 **直接** 相连。房间 `0` 已经构筑完成，所以 `prevRoom[0] = -1` 。扩建计划中还有一条硬性要求，在完成所有房间的构筑之后，从房间 `0` 可以访问到每个房间。

你一次只能构筑 **一个** 房间。你可以在 **已经构筑好的** 房间之间自由穿行，只要这些房间是 **相连的** 。如果房间 `prevRoom[i]` 已经构筑完成，那么你就可以构筑房间 `i`。

返回你构筑所有房间的 **不同顺序的数目** 。由于答案可能很大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/19/d1.JPG)
<pre>
<strong>输入:</strong> prevRoom = [-1,0,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 仅有一种方案可以完成所有房间的构筑：0 → 1 → 2
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/06/19/d2.JPG)
<pre>
<strong>输入:</strong> prevRoom = [-1,0,0,1,2]
<strong>输出:</strong> 6
<strong>解释:</strong>
有 6 种不同顺序：
0 → 1 → 3 → 2 → 4
0 → 2 → 4 → 1 → 3
0 → 1 → 2 → 3 → 4
0 → 1 → 2 → 4 → 3
0 → 2 → 1 → 3 → 4
0 → 2 → 1 → 4 → 3
</pre>

#### 提示:
* `n == prevRoom.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `prevRoom[0] == -1`
* 对于所有的 `1 <= i < n` ，都有 `0 <= prevRoom[i] < n`
* 题目保证所有房间都构筑完成后，从房间 `0` 可以访问到每个房间

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    MOD = 1000000007
    factorial = [1]

    def waysToBuildRooms(self, prevRoom: List[int]) -> int:
        n = len(prevRoom)
        indegree = [0] * n
        subtreenodes = [0] * n
        orders = [1] * n

        while len(self.factorial) < n + 1:
            self.factorial.append(
                self.factorial[-1] * len(self.factorial) % self.MOD)

        for i in range(1, n):
            indegree[prevRoom[i]] += 1

        nodes = [i for i in range(n) if indegree[i] == 0]

        while nodes[-1] != 0:
            i = nodes.pop()
            j = prevRoom[i]
            indegree[j] -= 1
            if indegree[j] == 0:
                nodes.append(j)
            subtreenodes[i] += 1
            subtreenodes[j] += subtreenodes[i]
            orders[j] = orders[j] * orders[i] % self.MOD
            orders[j] = orders[j] * self.factorial[subtreenodes[j]] % self.MOD
            orders[j] = orders[j] * \
                pow(self.factorial[subtreenodes[i]],
                    self.MOD - 2, self.MOD) % self.MOD
            orders[j] = orders[j] * pow(self.factorial[subtreenodes[j] -
                                        subtreenodes[i]], self.MOD - 2, self.MOD) % self.MOD

        return orders[0]
```
