# @param {String} s
# @return {Integer}
def num_decodings(s)
    return 0 if s[0] == '0'

    prev, curr = 1, 1

    for i in 1...s.length
        if s[i] == '0' and (s[i - 1] > '2' or s[i - 1] == '0')
            return 0
        elsif s[i] == '0'
            curr = prev
        elsif s[i - 1..i] > "10" and s[i - 1..i] < "27"
            prev, curr = curr, prev + curr
        else
            prev = curr
        end
    end

    return curr
end
