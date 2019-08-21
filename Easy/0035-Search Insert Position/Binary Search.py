class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if not nums or target <= nums[0]:
            return 0
        if target > nums[-1]:
            return len(nums)

        left = 1
        right = len(nums) - 1
        while left <= right:
            mid = (left + right) // 2

            if target == nums[mid]:
                return mid
            elif target > nums[mid]:
                left = mid + 1
                if target < nums[left]:
                    return left
            elif target < nums[mid]:
                right = mid - 1
                if target > nums[right]:
                    return mid
