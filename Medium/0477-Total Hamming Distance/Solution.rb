# @param {Integer[]} nums
# @return {Integer}
def total_hamming_distance(nums)
    ret = 0

    for i in 0...30
        zeros, ones = 0, 0
        for num in nums
            if (1 << i) & num == 0
                zeros += 1
            else
                ones += 1
            end
        end
        ret += zeros * ones
    end

    return ret
end
