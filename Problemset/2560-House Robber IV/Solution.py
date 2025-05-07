class Solution:
    def minCapability(self, nums: List[int], k: int) -> int:
        def maxStolenHouses(capability: int) -> int:
            count = 0
            ret = 0

            for i in range(len(nums) + 1):
                if i < len(nums) and nums[i] <= capability:
                    count += 1
                else:
                    ret += (count + 1) // 2
                    count = 0

            return ret

        return bisect.bisect_left(range(max(nums) + 1), k, key=maxStolenHouses)
