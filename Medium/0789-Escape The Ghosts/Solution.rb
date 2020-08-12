# @param {Integer[][]} ghosts
# @param {Integer[]} target
# @return {Boolean}
def escape_ghosts(ghosts, target)
    min_distance = 20001

    ghosts.each do |ghost|
        distance = (ghost[0] - target[0]).abs + (ghost[1] - target[1]).abs
        min_distance = distance if distance < min_distance
    end

    return min_distance > target[0].abs + target[1].abs
end
