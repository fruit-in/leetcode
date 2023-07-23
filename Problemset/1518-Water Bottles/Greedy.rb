# @param {Integer} num_bottles
# @param {Integer} num_exchange
# @return {Integer}
def num_water_bottles(num_bottles, num_exchange)
    ret = num_empty = num_bottles

    while num_empty >= num_exchange
        ret += num_empty / num_exchange
        num_empty = num_empty % num_exchange + num_empty / num_exchange
    end

    return ret
end
