# @param {Integer[]} nums
# @return {Integer}
def single_non_duplicate(nums)
    l, r = 0, nums.length - 1

    while l <= r
        m = (l + r) / 2

        if m.even?
            if m < nums.length - 1 and nums[m] == nums[m + 1]
                l = m + 1
            elsif m > 0 and nums[m] == nums[m - 1]
                r = m - 1
            else
                return nums[m]
            end
        else
            if nums[m] == nums[m + 1]
                r = m - 1
            elsif nums[m] == nums[m - 1]
                l = m + 1
            else
                return nums[m]
            end
        end
    end
end
