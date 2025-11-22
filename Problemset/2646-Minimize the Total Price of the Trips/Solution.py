class Solution:
    def minimumTotalPrice(self, n: int, edges: List[List[int]], price: List[int], trips: List[List[int]]) -> int:
        def searchAndCount(start: int, end: int, prev: int = -1) -> bool:
            if start == end:
                count[start] += 1
                return True

            for neighbor in neighbors[start]:
                if neighbor != prev and searchAndCount(neighbor, end, start):
                    count[start] += 1
                    return True

            return False

        @cache
        def calculatePrice(root: int, canhalve: bool, prev: int = -1) -> int:
            price1 = inf
            price2 = price[root] * count[root]

            if canhalve and count[root] > 0:
                price1 = price2 // 2

                for neighbor in neighbors[root]:
                    if neighbor != prev:
                        price1 += calculatePrice(neighbor, False, root)

            for neighbor in neighbors[root]:
                if neighbor != prev:
                    price2 += min(calculatePrice(neighbor, False, root),
                                  calculatePrice(neighbor, True, root))

            return min(price1, price2)

        neighbors = [[] for _ in range(n)]
        count = [0] * n

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for start, end in trips:
            searchAndCount(start, end)

        return calculatePrice(0, True)
