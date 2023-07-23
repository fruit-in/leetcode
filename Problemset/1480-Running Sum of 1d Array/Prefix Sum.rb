# @param {Integer[]} nums
# @return {Integer[]}
def running_sum(nums)
    for i in 1...nums.length
        nums[i] += nums[i - 1]
    end

    return nums
end
