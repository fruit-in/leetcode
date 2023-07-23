# 1029. 两地调度
公司计划面试 ```2N``` 人。第 ```i``` 人飞往 ```A``` 市的费用为 ```costs[i][0]```，飞往 ```B``` 市的费用为 ```costs[i][1]```。

返回将每个人都飞到某座城市的最低费用，要求每个城市都有 ```N``` 人抵达。

#### 示例:
<pre>
<strong>输入:</strong> [[10,20],[30,200],[400,50],[30,20]]
<strong>输出:</strong> 110
<strong>解释:</strong>
第一个人去 A 市，费用为 10。
第二个人去 A 市，费用为 30。
第三个人去 B 市，费用为 50。
第四个人去 B 市，费用为 20。

最低总费用为 10 + 30 + 50 + 20 = 110，每个城市都有一半的人在面试。
</pre>

#### 提示:
1. ```1 <= costs.length <= 100```
2. ```costs.length``` 为偶数
3. ```1 <= costs[i][0], costs[i][1] <= 1000```

## 题解 (Python)

### 1. 题解
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
