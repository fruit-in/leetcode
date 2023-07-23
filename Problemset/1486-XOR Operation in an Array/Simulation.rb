# @param {Integer} n
# @param {Integer} start
# @return {Integer}
def xor_operation(n, start)
    ret = 0

    (0...n).each do |i|
        ret ^= start + 2 * i
    end

    return ret
end
