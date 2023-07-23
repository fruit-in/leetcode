# @param {Integer[]} arr
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def count_good_triplets(arr, a, b, c)
    ret = 0

    for i in 0...arr.length
        for j in (i + 1)...arr.length
            if (arr[i] - arr[j]).abs <= a
                for k in (j + 1)...arr.length
                    if (arr[j] - arr[k]).abs <= b and (arr[i] - arr[k]).abs <= c
                        ret += 1
                    end
                end
            end
        end
    end

    return ret
end
