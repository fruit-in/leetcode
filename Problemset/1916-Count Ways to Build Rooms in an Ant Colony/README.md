# 1916. Count Ways to Build Rooms in an Ant Colony
You are an ant tasked with adding `n` new rooms numbered `0` to `n-1` to your colony. You are given the expansion plan as a **0-indexed** integer array of length `n`, `prevRoom`, where `prevRoom[i]` indicates that you must build room `prevRoom[i]` before building room `i`, and these two rooms must be connected **directly**. Room `0` is already built, so `prevRoom[0] = -1`. The expansion plan is given such that once all the rooms are built, every room will be reachable from room `0`.

You can only build **one room** at a time, and you can travel freely between rooms you have **already built** only if they are **connected**. You can choose to build **any room** as long as its **previous room** is already built.

Return *the **number of different orders** you can build all the rooms in*. Since the answer may be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/19/d1.JPG)
<pre>
<strong>Input:</strong> prevRoom = [-1,0,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one way to build the additional rooms: 0 → 1 → 2
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/06/19/d2.JPG)
<pre>
<strong>Input:</strong> prevRoom = [-1,0,0,1,2]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 6 ways are:
0 → 1 → 3 → 2 → 4
0 → 2 → 4 → 1 → 3
0 → 1 → 2 → 3 → 4
0 → 1 → 2 → 4 → 3
0 → 2 → 1 → 3 → 4
0 → 2 → 1 → 4 → 3
</pre>

#### Constraints:
* `n == prevRoom.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `prevRoom[0] == -1`
* `0 <= prevRoom[i] < n` for `all 1 <= i < n`
* Every room is reachable from room `0` once all the rooms are built.

## Solutions (Python)

### 1. Solution
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
