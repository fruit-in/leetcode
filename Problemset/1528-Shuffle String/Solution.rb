# @param {String} s
# @param {Integer[]} indices
# @return {String}
def restore_string(s, indices)
    ret = [''] * s.length

    for i in 0...s.length
        ret[indices[i]] = s[i]
    end

    return ret.join()
end
