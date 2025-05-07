class Solution:
    def numOfWays(self, nums: List[int]) -> int:
        def numOfWaysIncludingSame(nums: List[int]) -> int:
            if len(nums) <= 2:
                return 1

            left = []
            right = []

            for i in range(1, len(nums)):
                if nums[i] < nums[0]:
                    left.append(nums[i])
                else:
                    right.append(nums[i])

            return math.comb(len(nums) - 1, len(left)) % 1000000007 * numOfWaysIncludingSame(left) % 1000000007 * numOfWaysIncludingSame(right) % 1000000007

        return (numOfWaysIncludingSame(nums) - 1) % 1000000007
