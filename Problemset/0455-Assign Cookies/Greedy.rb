# @param {Integer[]} g
# @param {Integer[]} s
# @return {Integer}
def find_content_children(g, s)
    i, j = 0, 0
    ret = 0

    g.sort!
    s.sort!

    while i < g.length and j < s.length
        if s[j] >= g[i]
            ret += 1
            i += 1
        end
        j += 1
    end

    return ret
end
