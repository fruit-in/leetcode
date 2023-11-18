# 1847. Closest Room
There is a hotel with `n` rooms. The rooms are represented by a 2D integer array `rooms` where <code>rooms[i] = [roomId<sub>i</sub>, size<sub>i</sub>]</code> denotes that there is a room with room number <code>roomId<sub>i</sub></code> and size equal to <code>size<sub>i</sub></code>. Each <code>roomId<sub>i</sub></code> is guaranteed to be **unique**.

You are also given `k` queries in a 2D array `queries` where <code>queries[j] = [preferred<sub>j</sub>, minSize<sub>j</sub>]</code>. The answer to the <code>j<sup>th</sup></code> query is the room number `id` of a room such that:

* The room has a size of **at least** <code>minSize<sub>j</sub></code>, and
* <code>abs(id - preferred<sub>j</sub>)</code> is **minimized**, where `abs(x)` is the absolute value of x.

If there is a **tie** in the absolute difference, then use the room with the **smallest** such `id`. If there is **no such room**, the answer is `-1`.

Return *an array* `answer` *of length* `k` *where* `answer[j]` *contains the answer to the* <code>j<sup>th</sup></code> *query*.

#### Example 1:
<pre>
<strong>Input:</strong> rooms = [[2,2],[1,2],[3,2]], queries = [[3,1],[3,3],[5,2]]
<strong>Output:</strong> [3,-1,3]
<strong>Explanation:</strong> The answers to the queries are as follows:
Query = [3,1]: Room number 3 is the closest as abs(3 - 3) = 0, and its size of 2 is at least 1. The answer is 3.
Query = [3,3]: There are no rooms with a size of at least 3, so the answer is -1.
Query = [5,2]: Room number 3 is the closest as abs(3 - 5) = 2, and its size of 2 is at least 2. The answer is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rooms = [[1,4],[2,3],[3,5],[4,1],[5,2]], queries = [[2,3],[2,4],[2,5]]
<strong>Output:</strong> [2,1,3]
<strong>Explanation:</strong> The answers to the queries are as follows:
Query = [2,3]: Room number 2 is the closest as abs(2 - 2) = 0, and its size of 3 is at least 3. The answer is 2.
Query = [2,4]: Room numbers 1 and 3 both have sizes of at least 4. The answer is 1 since it is smaller.
Query = [2,5]: Room number 3 is the only room with a size of at least 5. The answer is 3.
</pre>

#### Constraints:
* `n == rooms.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `k == queries.length`
* <code>1 <= k <= 10<sup>4</sup></code>
* <code>1 <= roomId<sub>i</sub>, preferred<sub>j</sub> <= 10<sup>7</sup></code>
* <code>1 <= size<sub>i</sub>, minSize<sub>j</sub> <= 10<sup>7</sup></code>

## Solutions (Python)

### 1. Solution
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
