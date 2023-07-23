# @param {String} s
# @return {Integer}
def count_binary_substrings(s)
    prev, curr = 0, 0
    ret = 0

    for i in 0...s.length
        curr += 1
        if i == s.length - 1 or s[i] != s[i + 1]
            ret += [prev, curr].min
            prev, curr = curr, 0
        end
    end

    return ret
end
