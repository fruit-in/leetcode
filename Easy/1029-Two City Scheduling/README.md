# 1029. Two City Scheduling
There are <code>2N</code> people a company is planning to interview. The cost of flying the <code>i</code>-th person to city <code>A</code> is <code>costs[i][0]</code>, and the cost of flying the <code>i</code>-th person to city <code>B</code> is <code>costs[i][1]</code>.

Return the minimum cost to fly every person to a city such that exactly <code>N</code> people arrive in each city.

#### Example 1:
<pre>
<strong>Input:</strong> [[10,20],[30,200],[400,50],[30,20]]
<strong>Output:</strong> 110
<strong>Explanation:</strong> 
The first person goes to city A for a cost of 10.
The second person goes to city A for a cost of 30.
The third person goes to city B for a cost of 50.
The fourth person goes to city B for a cost of 20.

The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.
</pre>

#### Note:
1. <code>1 <= costs.length <= 100</code>
2. It is guaranteed that <code>costs.length</code> is even.
3. <code>1 <= costs[i][0], costs[i][1] <= 1000</code>

## Solutions

### 1. Solution (Python3)
```Python3
class Solution:
    def twoCitySchedCost(self, costs: List[List[int]]) -> int:
        N = len(costs) / 2
        A, B = [], []
        costs.sort(key=lambda cost : abs(cost[0] - cost[1]), reverse=True)
        for cost in costs:
            if (cost[0] <= cost[1] and len(A) < N) or len(B) == N:
                A.append(cost)
            elif (cost[0] >= cost[1] and len(B) < N) or len(A) == N:
                B.append(cost)
        return sum(cost[0] for cost in A) + sum(cost[1] for cost in B)
```
