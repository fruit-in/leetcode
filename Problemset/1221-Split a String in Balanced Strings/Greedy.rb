# @param {String} s
# @return {Integer}
def balanced_string_split(s)
    cnt = 0
    ret = 0

    for ch in s.chars
        if ch == 'R'
            cnt += 1
        elsif ch == 'L'
            cnt -= 1
        end

        if cnt == 0
            ret += 1
        end
    end

    return ret
end
