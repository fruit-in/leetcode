class Solution:
    def maximumTastiness(self, price: List[int], k: int) -> int:
        def cannotKCandies(tastiness: int) -> bool:
            i = 0
            count = 0

            while i < len(price) and count < k:
                count += 1
                i = bisect.bisect_left(price, price[i] + tastiness, lo=i + 1)

            return count < k

        price.sort()

        return bisect.bisect(range(price[-1] - price[0] + 1), False, key=cannotKCandies) - 1
