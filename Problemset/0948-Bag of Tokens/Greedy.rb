# @param {Integer[]} tokens
# @param {Integer} p
# @return {Integer}
def bag_of_tokens_score(tokens, p)
    return 0 if tokens.length == 0

    i, j = 0, tokens.length - 1
    score = 0
    ret = 0
    tokens.sort!

    while i <= j
        if p >= tokens[i]
            p -= tokens[i]
            score += 1
            ret = [ret, score].max
            i += 1
        elsif score > 0
            p += tokens[j]
            score -= 1
            j -= 1
        else
            break
        end
    end

    return ret
end
