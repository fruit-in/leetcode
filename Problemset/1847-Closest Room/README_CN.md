# 1847. 最近的房间
一个酒店里有 `n` 个房间，这些房间用二维整数数组 `rooms` 表示，其中 <code>rooms[i] = [roomId<sub>i</sub>, size<sub>i</sub>]</code> 表示有一个房间号为 <code>roomId<sub>i</sub></code> 的房间且它的面积为 <code>size<sub>i</sub></code> 。每一个房间号 <code>roomId<sub>i</sub></code> 保证是 **独一无二** 的。

同时给你 `k` 个查询，用二维数组 `queries` 表示，其中 <code>queries[j] = [preferred<sub>j</sub>, minSize<sub>j</sub>]</code> 。第 `j` 个查询的答案是满足如下条件的房间 `id` ：

* 房间的面积 **至少** 为 <code>minSize<sub>j</sub></code> ，且
* <code>abs(id - preferred<sub>j</sub>)</code> 的值 **最小** ，其中 `abs(x)` 是 `x` 的绝对值。

如果差的绝对值有 **相等** 的，选择 **最小** 的 `id` 。如果 **没有满足条件的房间** ，答案为 `-1` 。

请你返回长度为 `k` 的数组 `answer` ，其中 `answer[j]` 为第 `j` 个查询的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> rooms = [[2,2],[1,2],[3,2]], queries = [[3,1],[3,3],[5,2]]
<strong>输出:</strong> [3,-1,3]
<strong>解释:</strong> 查询的答案如下：
查询 [3,1] ：房间 3 的面积为 2 ，大于等于 1 ，且号码是最接近 3 的，为 abs(3 - 3) = 0 ，所以答案为 3 。
查询 [3,3] ：没有房间的面积至少为 3 ，所以答案为 -1 。
查询 [5,2] ：房间 3 的面积为 2 ，大于等于 2 ，且号码是最接近 5 的，为 abs(3 - 5) = 2 ，所以答案为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rooms = [[1,4],[2,3],[3,5],[4,1],[5,2]], queries = [[2,3],[2,4],[2,5]]
<strong>输出:</strong> [2,1,3]
<strong>解释:</strong> 查询的答案如下：
查询 [2,3] ：房间 2 的面积为 3 ，大于等于 3 ，且号码是最接近的，为 abs(2 - 2) = 0 ，所以答案为 2 。
查询 [2,4] ：房间 1 和 3 的面积都至少为 4 ，答案为 1 因为它房间编号更小。
查询 [2,5] ：房间 3 是唯一面积大于等于 5 的，所以答案为 3 。
</pre>

#### 提示:
* `n == rooms.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `k == queries.length`
* <code>1 <= k <= 10<sup>4</sup></code>
* <code>1 <= roomId<sub>i</sub>, preferred<sub>j</sub> <= 10<sup>7</sup></code>
* <code>1 <= size<sub>i</sub>, minSize<sub>j</sub> <= 10<sup>7</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from sortedcontainers import SortedList


class Solution:
    def closestRoom(self, rooms: List[List[int]], queries: List[List[int]]) -> List[int]:
        queries = list(enumerate(queries))
        roomids = SortedList()
        answer = [-1] * len(queries)

        rooms.sort(key=lambda x: x[1])
        queries.sort(key=lambda x: x[1][1], reverse=True)

        for (j, [prefered, minsize]) in queries:
            while len(rooms) > 0 and rooms[-1][1] >= minsize:
                roomids.add(rooms.pop()[0])
            if len(roomids) == 0:
                continue

            i = roomids.bisect_left(prefered)
            if i == 0:
                answer[j] = roomids[0]
            elif i == len(roomids):
                answer[j] = roomids[-1]
            elif abs(roomids[i] - prefered) < abs(roomids[i - 1] - prefered):
                answer[j] = roomids[i]
            else:
                answer[j] = roomids[i - 1]

        return answer
```
