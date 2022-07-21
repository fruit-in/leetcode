class Solution:
    def canReorderDoubled(self, arr: List[int]) -> bool:
        count = {}
        nums = []

        for x in arr:
            if x % 2 == 0:
                if x not in count:
                    count[x] = 0
                count[x] += 1
            else:
                if 2 * x not in count:
                    count[2 * x] = 0
                count[2 * x] -= 1

        for x, v in count.items():
            if v < 0:
                return False
            elif v > 0:
                nums.append(x)

        nums.sort(key=lambda x: (x >= 0, abs(x)))

        for x in nums:
            if count[x] == 0:
                continue
            elif 2 * x in count and count[x] <= count[2 * x]:
                count[2 * x] -= count[x]
                count[x] = 0
            else:
                return False

        return True
