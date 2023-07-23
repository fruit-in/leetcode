# @param {Integer} p
# @param {Integer} q
# @return {Integer}
def mirror_reflection(p, q)
    a, b = p, q

    while b != 0
        a, b = b, a % b
    end

    if p / a % 2 == 0
        return 2
    elsif q / a % 2 == 0
        return 0
    else
        return 1
    end
end
