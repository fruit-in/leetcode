# @param {Integer[]} nums
# @return {Integer[]}
def decompress_rl_elist(nums)
    ret = []

    for i in (0...nums.length).step(2)
        ret.concat([nums[i + 1]] * nums[i])
    end

    return ret
end
