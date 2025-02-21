# 2092. Find All People With Secret
You are given an integer `n` indicating there are `n` people numbered from `0` to `n - 1`. You are also given a **0-indexed** 2D integer array `meetings` where <code>meetings[i] = [x<sub>i</sub>, y<sub>i</sub>, time<sub>i</sub>]</code> indicates that person <code>x<sub>i</sub></code> and person <code>y<sub>i</sub></code> have a meeting at <code>time<sub>i</sub></code>. A person may attend **multiple meetings** at the same time. Finally, you are given an integer firstPerson.

Person `0` has a **secret** and initially shares the secret with a person `firstPerson` at time `0`. This secret is then shared every time a meeting takes place with a person that has the secret. More formally, for every meeting, if a person <code>x<sub>i</sub></code> has the secret at <code>time<sub>i</sub></code>, then they will share the secret with person <code>y<sub>i</sub></code>, and vice versa.

The secrets are shared **instantaneously**. That is, a person may receive the secret and share it with people in other meetings within the same time frame.

Return *a list of all the people that have the secret after all the meetings have taken place*. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 6, meetings = [[1,2,5],[2,3,8],[1,5,10]], firstPerson = 1
<strong>Output:</strong> [0,1,2,3,5]
<strong>Explanation:</strong>
At time 0, person 0 shares the secret with person 1.
At time 5, person 1 shares the secret with person 2.
At time 8, person 2 shares the secret with person 3.
At time 10, person 1 shares the secret with person 5.
Thus, people 0, 1, 2, 3, and 5 know the secret after all the meetings.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, meetings = [[3,1,3],[1,2,2],[0,3,3]], firstPerson = 3
<strong>Output:</strong> [0,1,3]
<strong>Explanation:</strong>
At time 0, person 0 shares the secret with person 3.
At time 2, neither person 1 nor person 2 know the secret.
At time 3, person 3 shares the secret with person 0 and person 1.
Thus, people 0, 1, and 3 know the secret after all the meetings.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 5, meetings = [[3,4,2],[1,2,1],[2,3,1]], firstPerson = 1
<strong>Output:</strong> [0,1,2,3,4]
<strong>Explanation:</strong>
At time 0, person 0 shares the secret with person 1.
At time 1, person 1 shares the secret with person 2, and person 2 shares the secret with person 3.
Note that person 2 can share the secret at the same time as receiving it.
At time 2, person 3 shares the secret with person 4.
Thus, people 0, 1, 2, 3, and 4 know the secret after all the meetings.
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= meetings.length <= 10<sup>5</sup></code>
* `meetings[i].length == 3`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* <code>1 <= time<sub>i</sub> <= 10<sup>5</sup></code>
* `1 <= firstPerson <= n - 1`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findAllPeople(self, n: int, meetings: List[List[int]], firstPerson: int) -> List[int]:
        meetings.sort(key=lambda m: m[2])
        knows = {0, firstPerson}
        currknows = set()
        edges = {}

        for i in range(len(meetings)):
            x, y, time = meetings[i]

            if x in knows or y in knows:
                knows.add(x)
                knows.add(y)
                currknows.add(x)
                currknows.add(y)
            else:
                if x not in edges:
                    edges[x] = []
                if y not in edges:
                    edges[y] = []
                edges[x].append(y)
                edges[y].append(x)

            if i + 1 == len(meetings) or time != meetings[i + 1][2]:
                currknows = list(currknows)
                while currknows != []:
                    x = currknows.pop()
                    for y in edges.get(x, []):
                        if y not in knows:
                            knows.add(y)
                            currknows.append(y)
                currknows = set()
                edges = {}

        return list(knows)
```
