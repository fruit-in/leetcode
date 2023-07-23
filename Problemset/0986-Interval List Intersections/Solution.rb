# @param {Integer[][]} a
# @param {Integer[][]} b
# @return {Integer[][]}
def interval_intersection(a, b)
    i, j = 0, 0
    ret = []

    while i < a.length and j < b.length
        max_l = [a[i][0], b[j][0]].max
        min_r = [a[i][1], b[j][1]].min

        if min_r >= max_l
            ret.push([max_l, min_r])
        end

        if min_r == a[i][1]
            i += 1
        else
            j += 1
        end
    end

    return ret
end
