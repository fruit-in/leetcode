class Solution:
    def maximumANDSum(self, nums: List[int], numSlots: int) -> int:
        @cache
        def dfs(i: int, slots: Tuple[int]) -> int:
            if i == len(nums):
                return 0

            slots = list(slots)
            ret = 0

            for j in range(numSlots):
                if slots[j] < 2:
                    slots[j] += 1
                    ret = max(ret, (nums[i] & (j + 1)) +
                              dfs(i + 1, tuple(slots)))
                    slots[j] -= 1

            return ret

        return dfs(0, tuple([0] * numSlots))
