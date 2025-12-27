class Solution:
    def subarrayLCM(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            x = 1
            for j in range(i, len(nums)):
                x = lcm(x, nums[j])
                if x == k:
                    ret += 1
                elif x > k:
                    break

        return ret
