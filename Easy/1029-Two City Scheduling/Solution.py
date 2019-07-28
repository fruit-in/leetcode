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
