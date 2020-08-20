# @param {Integer} num
# @return {Boolean}
def is_ugly(num)
    if num == 0
        return false
    elsif num == 1
        return true
    elsif num % 2 == 0
        return is_ugly(num / 2)
    elsif num % 3 == 0
        return is_ugly(num / 3)
    elsif num % 5 == 0
        return is_ugly(num / 5)
    else
        return false
    end
end
