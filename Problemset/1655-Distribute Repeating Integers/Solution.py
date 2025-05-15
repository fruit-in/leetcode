class Solution:
    def canDistribute(self, nums: List[int], quantity: List[int]) -> bool:
        def dfs(i: int) -> bool:
            if i == len(quantity):
                return True

            for j in range(len(count)):
                if count[j] >= quantity[i] and (j == 0 or count[j] != count[j - 1]):
                    count[j] -= quantity[i]
                    if dfs(i + 1):
                        return True
                    count[j] += quantity[i]

            return False

        count = sorted(collections.Counter(nums).values())[-len(quantity):]

        if count[-1] < max(quantity) or sum(count) < sum(quantity):
            return False

        return dfs(0)
