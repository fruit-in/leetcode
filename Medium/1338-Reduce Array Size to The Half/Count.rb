# @param {Integer[]} arr
# @return {Integer}
def min_set_size(arr)
    cnt = Hash.new(0)

    for num in arr
        cnt[num] += 1
    end

    cnt = cnt.values.sort

    rm = 0
    ret = 0

    while rm < arr.length / 2
        ret += 1
        rm += cnt.pop
    end

    return ret
end
