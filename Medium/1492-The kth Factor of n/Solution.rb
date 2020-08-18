# @param {Integer} n
# @param {Integer} k
# @return {Integer}
def kth_factor(n, k)
    factor = 1
    i = 0

    while factor * factor <= n
        if n % factor == 0
            i += 1
            return factor if i == k
        end
        factor += 1
    end

    factor -= 1
    factor -= 1 if factor * factor == n

    while factor > 0
        if n % factor == 0
            i += 1
            return n / factor if i == k
        end
        factor -= 1
    end

    return -1
end
