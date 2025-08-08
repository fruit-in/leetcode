class Solution:
    def subarrayGCD(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            maxgcd = nums[i]

            for j in range(i, len(nums)):
                maxgcd = gcd(maxgcd, nums[j])
                if maxgcd % k != 0:
                    break
                elif maxgcd == k:
                    ret += 1

        return ret
