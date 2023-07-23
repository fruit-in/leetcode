# @param {String[]} logs
# @return {Integer}
def min_operations(logs)
    ret = 0

    for log in logs
        if log == "../" and ret > 0
            ret -= 1
        elsif log != "../" and log != "./"
            ret += 1
        end
    end

    return ret
end
