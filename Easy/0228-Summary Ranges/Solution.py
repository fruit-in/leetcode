class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        i = 0
        ret = []

        for j in range(len(nums)):
            if j == len(nums) - 1 or nums[j] + 1 != nums[j + 1]:
                if i == j:
                    ret.append(str(nums[i]))
                else:
                    ret.append(str(nums[i]) + "->" + str(nums[j]))
                i = j + 1

        return ret
