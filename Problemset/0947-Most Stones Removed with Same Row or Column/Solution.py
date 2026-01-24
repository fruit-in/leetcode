class Solution:
    def removeStones(self, stones: List[List[int]]) -> int:
        n = len(stones)
        parent = {(x, y): (x, y) for x, y in stones}
        group = set()
        stones.sort()

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if xi != xj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        stones.sort(key=lambda s: s[1])

        for i in range(1, n):
            xi, yi = stones[i]
            xj, yj = stones[i - 1]

            if yi != yj:
                continue

            while parent[(xi, yi)] != parent[parent[(xi, yi)]]:
                parent[(xi, yi)] = parent[parent[(xi, yi)]]
            while parent[(xj, yj)] != parent[parent[(xj, yj)]]:
                parent[(xj, yj)] = parent[parent[(xj, yj)]]

            parent[parent[(xi, yi)]] = parent[(xj, yj)]

        for x, y in stones:
            while parent[(x, y)] != parent[parent[(x, y)]]:
                parent[(x, y)] = parent[parent[(x, y)]]

            group.add(parent[(x, y)])

        return n - len(group)
