# @param {Integer} n
# @return {String}
def count_and_say(n)
    s = "1"

    for _ in 1...n
        tmp = ""
        i = 0

        for j in 0...s.length
            if s[i] != s[j]
                tmp += (j - i).to_s + s[i]
                i = j
            end
        end

        tmp += (s.length - i).to_s + s[i]
        s = tmp
    end

    return s
end
