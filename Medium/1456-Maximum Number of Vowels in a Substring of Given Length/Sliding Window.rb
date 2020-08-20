# @param {String} s
# @param {Integer} k
# @return {Integer}
def max_vowels(s, k)
    i = 0
    cnt = 0
    ret = 0

    for j in 0...s.length
        if "aeiou".include?(s[j])
            cnt += 1
        end
        if j - i == k
            if "aeiou".include?(s[i])
                cnt -= 1
            end
            i += 1
        end
        ret = [ret, cnt].max
    end

    return ret
end
