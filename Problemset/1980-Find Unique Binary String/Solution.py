class Solution:
    def findDifferentBinaryString(self, nums: List[str]) -> str:
        nums = {int(num, 2) for num in nums}

        for x in range(2 ** len(nums)):
            if x not in nums:
                return bin(x)[2:].zfill(len(nums))
