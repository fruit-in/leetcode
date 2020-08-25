# @param {Integer[]} arr
# @return {Boolean}
def can_make_arithmetic_progression(arr)
    arr.sort!

    for i in 2...arr.length
        return false if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2]
    end

    return true
end
