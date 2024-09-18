# 332. Reconstruct Itinerary
You are given a list of airline `tickets` where <code>tickets[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.

All of the tickets belong to a man who departs from `"JFK"`, thus, the itinerary must begin with `"JFK"`. If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.

* For example, the itinerary `["JFK", "LGA"]` has a smaller lexical order than `["JFK", "LGB"]`.

You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/14/itinerary1-graph.jpg)
<pre>
<strong>Input:</strong> tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
<strong>Output:</strong> ["JFK","MUC","LHR","SFO","SJC"]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/03/14/itinerary2-graph.jpg)
<pre>
<strong>Input:</strong> tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
<strong>Output:</strong> ["JFK","ATL","JFK","SFO","ATL","SFO"]
<strong>Explanation:</strong> Another possible reconstruction is ["JFK","SFO","ATL","JFK","ATL","SFO"] but it is larger in lexical order.
</pre>

#### Constraints:
* `1 <= tickets.length <= 300`
* `tickets[i].length == 2`
* <code>from<sub>i</sub>.length == 3</code>
* <code>to<sub>i</sub>.length == 3</code>
* <code>from<sub>i</sub></code> and <code>to<sub>i</sub></code> consist of uppercase English letters.
* <code>from<sub>i</sub> != to<sub>i</sub></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        targets = {}
        stack = []
        used = set()
        itinerary = [("JFK", None, None)]

        for fr0m, to in tickets:
            if fr0m not in targets:
                targets[fr0m] = []
            if to not in targets:
                targets[to] = []
            targets[fr0m].append(to)
        for fr0m in targets:
            targets[fr0m].sort(reverse=True)

        while True:
            fr0m = itinerary[-1][0]

            for i in range(len(targets[fr0m])):
                if (fr0m, i) not in used:
                    if stack == [] \
                            or fr0m != stack[-1][0] \
                            or targets[fr0m][i] != targets[fr0m][stack[-1][1]] \
                            or len(itinerary) != stack[-1][2]:
                        stack.append((fr0m, i, len(itinerary)))

            while stack[-1][2] < len(itinerary):
                _, fr0m, i = itinerary.pop()
                used.remove((fr0m, i))

            fr0m, i, _ = stack.pop()
            used.add((fr0m, i))
            itinerary.append((targets[fr0m][i], fr0m, i))

            if len(itinerary) == len(tickets) + 1:
                return [airport for airport, _, _ in itinerary]
```
