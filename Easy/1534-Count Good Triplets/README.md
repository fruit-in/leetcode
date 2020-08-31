# 1534. Count Good Triplets
Given an array of integers `arr`, and three integers `a`, `b` and `c`. You need to find the number of good triplets.

A triplet `(arr[i], arr[j], arr[k])` is **good** if the following conditions are true:
* `0 <= i < j < k < arr.length`
* `|arr[i] - arr[j]| <= a`
* `|arr[j] - arr[k]| <= b`
* `|arr[i] - arr[k]| <= c`

Where `|x|` denotes the absolute value of `x`.

Return *the number of good triplets*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 good triplets: [(3,0,1), (3,0,1), (3,1,1), (0,1,1)].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,2,2,3], a = 0, b = 0, c = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> No triplet satisfies all conditions.
</pre>

#### Constraints:
* `3 <= arr.length <= 100`
* `0 <= arr[i] <= 1000`
* `0 <= a, b, c <= 1000`

## Solutions (Ruby)

### 1. Brute Force
```Ruby
# @param {Integer[]} arr
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def count_good_triplets(arr, a, b, c)
    ret = 0

    for i in 0...arr.length
        for j in (i + 1)...arr.length
            if (arr[i] - arr[j]).abs <= a
                for k in (j + 1)...arr.length
                    if (arr[j] - arr[k]).abs <= b and (arr[i] - arr[k]).abs <= c
                        ret += 1
                    end
                end
            end
        end
    end

    return ret
end
```
