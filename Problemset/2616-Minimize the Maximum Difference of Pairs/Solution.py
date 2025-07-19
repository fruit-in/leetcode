class Solution:
    def minimizeMax(self, nums: List[int], p: int) -> int:
        def countPairs(maxdiff: int) -> int:
            i = 0
            ret = 0

            while i < len(diffs):
                if diffs[i] <= maxdiff:
                    ret += 1
                    i += 1
                i += 1

            return ret

        nums.sort()
        diffs = [nums[i + 1] - nums[i] for i in range(len(nums) - 1)]

        return bisect_left(range(0, max(diffs, default=0) + 1), p, key=countPairs)
