# @param {Integer[]} nums
# @return {Integer}
def pivot_index(nums)
    total_sum = nums.sum
    left_sum = 0

    for i in 0...nums.length
        return i if 2 * left_sum == total_sum - nums[i]
        left_sum += nums[i]
    end

    return -1
end
