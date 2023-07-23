# @param {Integer[]} customers
# @param {Integer[]} grumpy
# @param {Integer} x
# @return {Integer}
def max_satisfied(customers, grumpy, x)
    curr = 0
    ret = 0

    for i in 0...grumpy.length
        if i < x or grumpy[i] == 0
            curr += customers[i]
            ret += customers[i]
        end
        curr -= customers[i - x] if i >= x and grumpy[i - x] == 1
        curr += customers[i] if i >= x and grumpy[i] == 1

        ret = [ret, curr].max
    end

    return ret
end
