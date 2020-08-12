# @param {Integer[]} answers
# @return {Integer}
def num_rabbits(answers)
    same_tell = Hash.new(0)
    ret = 0

    for answer in answers
        same_tell[answer + 1] += 1
    end

    same_tell.each do |k, v|
        ret += (v * 1.0 / k).ceil * k
    end

    return ret
end
