# 881. Boats to Save People
The `i`-th person has weight `people[i]`, and each boat can carry a maximum weight of `limit`.

Each boat carries at most 2 people at the same time, provided the sum of the weight of those people is at most `limit`.

Return the minimum number of boats to carry every given person.  (It is guaranteed each person can be carried by a boat.)

#### Example 1:
<pre>
<b>Input:</b> people = [1,2], limit = 3
<b>Output:</b> 1
<b>Explanation:</b> 1 boat (1, 2)
</pre>

#### Example 2:
<pre>
<b>Input:</b> people = [3,2,2,1], limit = 3
<b>Output:</b> 3
<b>Explanation:</b> 3 boats (1, 2), (2) and (3)
</pre>

#### Example 3:
<pre>
<b>Input:</b> people = [3,5,3,4], limit = 5
<b>Output:</b> 4
<b>Explanation:</b> 4 boats (3), (3), (4), (5)
</pre>

#### Note:
* `1 <= people.length <= 50000`
* `1 <= people[i] <= limit <= 30000`

## Solutions (Ruby)

### 1. Greedy
```Ruby
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
```
