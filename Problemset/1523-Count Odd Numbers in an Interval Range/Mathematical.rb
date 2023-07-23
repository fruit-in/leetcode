# @param {Integer} low
# @param {Integer} high
# @return {Integer}
def count_odds(low, high)
    return (high - low + low % 2 + high % 2) / 2
end
