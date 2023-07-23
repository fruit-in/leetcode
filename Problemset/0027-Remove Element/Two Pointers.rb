# @param {Integer[]} nums
# @param {Integer} val
# @return {Integer}
def remove_element(nums, val)
    ret = 0

    for i in 0...nums.length
        if nums[i] != val
            nums[ret] = nums[i]
            ret += 1
        end
    end

    return ret
end
