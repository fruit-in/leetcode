# @param {Integer[]} gas
# @param {Integer[]} cost
# @return {Integer}
def can_complete_circuit(gas, cost)
    start = 0
    tank = 0

    for i in 0...(2 * gas.length)
        j = i % gas.length

        tank += gas[j] - cost[j]

        if tank >= 0 and (j + 1) % gas.length == start
            return start
        elsif tank < 0
            start = j + 1
            tank = 0
        end
    end

    return -1
end
