# @param {Integer} n
# @return {Integer}
def clumsy(n)
    if n % 4 == 0
        return [n + 1, 7].max
    elsif n < 3
        return n
    elsif n % 4 == 1 or n % 4 == 2
        return n + 2
    else
        return [n - 1, 6].max
    end
end
