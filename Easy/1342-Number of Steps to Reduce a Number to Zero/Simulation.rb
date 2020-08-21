# @param {Integer} num
# @return {Integer}
def number_of_steps (num)
    ret = 0

    while num != 0
        if num % 2 == 0
            num /= 2
        else
            num -= 1
        end
        ret += 1
    end

    return ret
end
