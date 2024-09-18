# 332. 重新安排行程
给你一份航线列表 `tickets` ，其中 <code>tickets[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> 表示飞机出发和降落的机场地点。请你对该行程进行重新规划排序。

所有这些机票都属于一个从 `JFK`（肯尼迪国际机场）出发的先生，所以该行程必须从 `JFK` 开始。如果存在多种有效的行程，请你按字典排序返回最小的行程组合。

* 例如，行程 `["JFK", "LGA"]` 与 `["JFK", "LGB"]` 相比就更小，排序更靠前。

假定所有机票至少存在一种合理的行程。且所有的机票 必须都用一次 且 只能用一次。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/14/itinerary1-graph.jpg)
<pre>
<strong>输入:</strong> tickets = [["MUC","LHR"],["JFK","MUC"],["SFO","SJC"],["LHR","SFO"]]
<strong>输出:</strong> ["JFK","MUC","LHR","SFO","SJC"]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/03/14/itinerary2-graph.jpg)
<pre>
<strong>输入:</strong> tickets = [["JFK","SFO"],["JFK","ATL"],["SFO","ATL"],["ATL","JFK"],["ATL","SFO"]]
<strong>输出:</strong> ["JFK","ATL","JFK","SFO","ATL","SFO"]
<strong>解释:</strong> 另一种有效的行程是 ["JFK","SFO","ATL","JFK","ATL","SFO"] ，但是它字典排序更大更靠后。
</pre>

#### 提示:
* `1 <= tickets.length <= 300`
* `tickets[i].length == 2`
* <code>from<sub>i</sub>.length == 3</code>
* <code>to<sub>i</sub>.length == 3</code>
* <code>from<sub>i</sub></code> 和 <code>to<sub>i</sub></code> 由大写英文字母组成
* <code>from<sub>i</sub> != to<sub>i</sub></code>

## 题解 (Python)

### 1. 题解
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
