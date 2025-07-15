class Solution:
    def maximumBeauty(self, flowers: List[int], newFlowers: int, target: int, full: int, partial: int) -> int:
        fullflowers = []
        partialflowers = []
        fullbeauty = 0
        maxbeauty = 0

        for flower in sorted(flowers, reverse=True):
            if flower >= target:
                fullbeauty += full
            elif target - flower <= newFlowers:
                fullbeauty += full
                newFlowers -= target - flower
                heappush(fullflowers, flower)
            else:
                heappush(partialflowers, (flower, 1))

        while True:
            while newFlowers > 0 and partialflowers and partialflowers[0][0] < target - 1:
                flower, count = heappop(partialflowers)
                while partialflowers and partialflowers[0][0] == flower:
                    count += heappop(partialflowers)[1]
                if partialflowers and (partialflowers[0][0] - flower) * count <= newFlowers:
                    newFlowers -= (partialflowers[0][0] - flower) * count
                    heappush(partialflowers, (partialflowers[0][0], count))
                elif partialflowers or (target - 1 - flower) * count > newFlowers:
                    flower += newFlowers // count
                    heappush(partialflowers,
                             (flower, count - newFlowers % count))
                    if newFlowers % count != 0:
                        heappush(partialflowers,
                                 (flower + 1, newFlowers % count))
                    newFlowers = 0
                else:
                    heappush(partialflowers, (target - 1, 1))

            beauty = fullbeauty
            if partialflowers:
                beauty += partialflowers[0][0] * partial
            maxbeauty = max(maxbeauty, beauty)

            if not fullflowers:
                break

            newFlowers += target - fullflowers[0]
            heappush(partialflowers, (heappop(fullflowers), 1))
            fullbeauty -= full

        return maxbeauty
