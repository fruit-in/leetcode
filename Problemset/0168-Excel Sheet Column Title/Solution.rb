# @param {Integer} n
# @return {String}
def convert_to_title(n)
    ret = ""

    while n > 0
        n -= 1
        ret = (n % 26 + 65).chr + ret
        n /= 26
    end

    return ret
end
