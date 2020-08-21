# @param {String} s
# @return {Integer}
def first_uniq_char(s)
    cnt = [0] * 26

    for i in 0...s.length
        cnt[s[i].ord - 'a'.ord] += 1
    end

    for i in 0...s.length
        return i if cnt[s[i].ord - 'a'.ord] == 1
    end

    return -1
end
