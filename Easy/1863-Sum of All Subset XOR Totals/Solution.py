class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        ret = 0

        for n in range(1, len(nums) + 1):
            for subset in combinations(nums, n):
                ret += reduce(lambda x, y: x ^ y, subset)

        return ret
