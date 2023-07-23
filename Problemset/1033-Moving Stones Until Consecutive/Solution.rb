# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer[]}
def num_moves_stones(a, b, c)
    x, y, z = [a, b, c].sort

    if z - x == 2
        return [0, 0]
    elsif z - y > 2 and y - x > 2
        return [2, z - x - 2]
    else
        return [1, z - x - 2]
    end
end
