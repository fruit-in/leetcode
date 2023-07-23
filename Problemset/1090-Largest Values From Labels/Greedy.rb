# @param {Integer[]} values
# @param {Integer[]} labels
# @param {Integer} num_wanted
# @param {Integer} use_limit
# @return {Integer}
def largest_vals_from_labels(values, labels, num_wanted, use_limit)
    items = Array.new(values.length) {|i| [values[i], labels[i]] }
    items.sort! {|a, b| b <=> a }
    label_use = Hash.new(0)
    cnt = 0
    ret = 0

    for item in items
        if label_use[item[1]] < use_limit
            ret += item[0]
            cnt += 1
            label_use[item[1]] += 1
            break if cnt >= num_wanted
        end
    end

    return ret
end
