# @param {Integer[]} arr
# @param {Integer} start
# @return {Boolean}
def can_reach(arr, start)
    positions = [start]

    while not positions.empty?
        i = positions.pop

        return true if arr[i] == 0

        if arr[i] > 0
            positions.push(i + arr[i]) if i + arr[i] < arr.length
            positions.push(i - arr[i]) if i - arr[i] >= 0
            arr[i] = -arr[i]
        end
    end

    return false
end
