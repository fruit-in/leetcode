# @param {Integer[]} people
# @param {Integer} limit
# @return {Integer}
def num_rescue_boats(people, limit)
    people.sort!
    i, j = 0, people.length - 1
    ret = 0

    while i <= j
        ret += 1
        i += 1 if people[j] + people[i] <= limit
        j -= 1
    end

    return ret
end
