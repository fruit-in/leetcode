# @param {Integer[]} nums
# @return {Integer}
def num_identical_pairs(nums)
    cnt = [0] * 101
    ret = 0

    for num in nums
        ret += cnt[num]
        cnt[num] += 1
    end

    return ret
end
