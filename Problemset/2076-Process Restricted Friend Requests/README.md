# 2076. Process Restricted Friend Requests
You are given an integer `n` indicating the number of people in a network. Each person is labeled from `0` to `n - 1`.

You are also given a **0-indexed** 2D integer array `restrictions`, where <code>restrictions[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> means that person <code>x<sub>i</sub></code> and person <code>y<sub>i</sub></code> **cannot** become **friends**, either **directly** or **indirectly** through other people.

Initially, no one is friends with each other. You are given a list of friend requests as a **0-indexed** 2D integer array `requests`, where <code>requests[j] = [u<sub>j</sub>, v<sub>j</sub>]</code> is a friend request between person <code>u<sub>j</sub></code> and person <code>v<sub>j</sub></code>.

A friend request is **successful** if <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> can be **friends**. Each friend request is processed in the given order (i.e., `requests[j]` occurs before `requests[j + 1]`), and upon a successful request, <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> **become direct friends** for all future friend requests.

Return *a **boolean array*** `result`, *where each* `result[j]` *is* `true` *if the* <code>j<sup>th</sup></code> *friend request is **successful** or* `false` *if it is not*.

**Note:** If <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> are already direct friends, the request is still **successful**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, restrictions = [[0,1]], requests = [[0,2],[2,1]]
<strong>Output:</strong> [true,false]
<strong>Explanation:</strong>
Request 0: Person 0 and person 2 can be friends, so they become direct friends.
Request 1: Person 2 and person 1 cannot be friends since person 0 and person 1 would be indirect friends (1--2--0).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, restrictions = [[0,1]], requests = [[1,2],[0,2]]
<strong>Output:</strong> [true,false]
<strong>Explanation:</strong>
Request 0: Person 1 and person 2 can be friends, so they become direct friends.
Request 1: Person 0 and person 2 cannot be friends since person 0 and person 1 would be indirect friends (0--2--1).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 5, restrictions = [[0,1],[1,2],[2,3]], requests = [[0,4],[1,2],[3,1],[3,4]]
<strong>Output:</strong> [true,false,true,false]
<strong>Explanation:</strong>
Request 0: Person 0 and person 4 can be friends, so they become direct friends.
Request 1: Person 1 and person 2 cannot be friends since they are directly restricted.
Request 2: Person 3 and person 1 can be friends, so they become direct friends.
Request 3: Person 3 and person 4 cannot be friends since person 0 and person 1 would be indirect friends (0--4--3--1).
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `0 <= restrictions.length <= 1000`
* `restrictions[i].length == 2`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* `1 <= requests.length <= 1000`
* `requests[j].length == 2`
* <code>0 <= u<sub>j</sub>, v<sub>j</sub> <= n - 1</code>
* <code>u<sub>j</sub> != v<sub>j</sub></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def friendRequests(self, n: int, restrictions: List[List[int]], requests: List[List[int]]) -> List[bool]:
        parent = {p: p for p in range(n)}
        group = {p: {p} for p in range(n)}
        result = [True] * len(requests)

        for i, [u, v] in enumerate(requests):
            for x, y in restrictions:
                if (x in group[parent[u]] and y in group[parent[v]]) or (y in group[parent[u]] and x in group[parent[v]]):
                    result[i] = False
                    break

            if result[i] and parent[u] != parent[v]:
                if len(group[parent[u]]) < len(group[parent[v]]):
                    u, v = v, u
                for p in group.pop(parent[v]):
                    parent[p] = parent[u]
                    group[parent[u]].add(p)

        return result
```
