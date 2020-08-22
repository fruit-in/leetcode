# @param {Integer} m
# @param {Integer} n
# @param {Integer[][]} ops
# @return {Integer}
def max_count(m, n, ops)
    min_a = ops.map {|op| op[0]}.min
    min_a = m if min_a.nil?
    min_b = ops.map {|op| op[1]}.min
    min_b = n if min_b.nil?

    return min_a * min_b
end
