# @param {Integer[]} nums
# @param {Integer[]} index
# @return {Integer[]}
def create_target_array(nums, index)
    target = Array.new

    for i in 0...nums.length
        target.insert(index[i], nums[i])
    end

    return target
end
