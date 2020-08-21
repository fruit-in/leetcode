# @param {String} s
# @return {String}
def reverse_vowels(s)
    vowels = "aiueoAIUEO"
    i, j = 0, s.length - 1

    while i < j
        while i < j and not vowels.include?(s[i])
            i += 1
        end
        while i < j and not vowels.include?(s[j])
            j -= 1
        end

        s[i], s[j] = s[j], s[i]

        i += 1
        j -= 1
    end

    return s
end
