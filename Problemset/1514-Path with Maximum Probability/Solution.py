import heapq


class Solution:
    def maxProbability(self, n: int, edges: List[List[int]], succProb: List[float], start_node: int, end_node: int) -> float:
        graph = {start_node: [(1, start_node)]}
        seen = set()
        heap = [(-1, start_node)]
        heapq.heapify(heap)

        for i in range(len(edges)):
            if edges[i][0] not in graph:
                graph[edges[i][0]] = []
            if edges[i][1] not in graph:
                graph[edges[i][1]] = []
            graph[edges[i][0]].append((succProb[i], edges[i][1]))
            graph[edges[i][1]].append((succProb[i], edges[i][0]))

        while len(heap) > 0:
            prob0, node0 = heapq.heappop(heap)
            seen.add(node0)

            if node0 == end_node:
                return -prob0

            for prob1, node1 in graph[node0]:
                if node1 not in seen:
                    heapq.heappush(heap, (prob0 * prob1, node1))

        return 0
