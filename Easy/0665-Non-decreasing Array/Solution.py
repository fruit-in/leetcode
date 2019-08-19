class Solution:
    def checkPossibility(self, nums: List[int]) -> bool:
        flag = False
        for i in range(len(nums) - 1):
            if nums[i] > nums[i + 1]:
                if flag:
                    return False
                flag = True
                if i > 0 and nums[i - 1] > nums[i + 1]:
                    nums[i + 1] = nums[i]
        return True
