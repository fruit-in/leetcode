# 1090. Largest Values From Labels
We have a set of items: the `i`-th item has value `values[i]` and label `labels[i]`.

Then, we choose a subset `S` of these items, such that:
* `|S| <= num_wanted`
* For every label `L`, the number of items in `S` with label `L` is `<= use_limit`.

Return the largest possible sum of the subset `S`.

#### Example 1:
<pre>
<strong>Input:</strong> values = [5,4,3,2,1], labels = [1,1,2,2,3], num_wanted = 3, use_limit = 1
<strong>Output:</strong> 9
<strong>Explanation:</strong> The subset chosen is the first, third, and fifth item.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> values = [5,4,3,2,1], labels = [1,3,3,3,2], num_wanted = 3, use_limit = 2
<strong>Output:</strong> 12
<strong>Explanation:</strong> The subset chosen is the first, second, and third item.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> values = [9,8,8,7,6], labels = [0,0,0,1,1], num_wanted = 3, use_limit = 1
<strong>Output:</strong> 16
<strong>Explanation:</strong> The subset chosen is the first and fourth item.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> values = [9,8,8,7,6], labels = [0,0,0,1,1], num_wanted = 3, use_limit = 2
<strong>Output:</strong> 24
<strong>Explanation:</strong> The subset chosen is the first, second, and fourth item.
</pre>

#### Note:
1. `1 <= values.length == labels.length <= 20000`
2. `0 <= values[i], labels[i] <= 20000`
3. `1 <= num_wanted, use_limit <= values.length`

## Solutions (Ruby)

### 1. Greedy
```Ruby
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
```
