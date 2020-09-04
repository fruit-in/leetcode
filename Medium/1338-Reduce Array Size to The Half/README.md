# 1338. Reduce Array Size to The Half
Given an array `arr`.  You can choose a set of integers and remove all the occurrences of these integers in the array.

Return *the minimum size of the set* so that **at least** half of the integers of the array are removed.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,3,3,3,5,5,5,2,2,7]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size of the old array).
Possible sets of size 2 are {3,5},{3,2},{5,2}.
Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has size greater than half of the size of the old array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [7,7,7,7,7,7]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible set you can choose is {7}. This will make the new array empty.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,9]
<strong>Output:</strong> 1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [1000,1000,3,7]
<strong>Output:</strong> 1
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6,7,8,9,10]
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `1 <= arr.length <= 10^5`
* `arr.length` is even.
* `1 <= arr[i] <= 10^5`

## Solutions (Ruby)

### 1. Count
```Ruby
# @param {Integer[]} arr
# @return {Integer}
def min_set_size(arr)
    cnt = Hash.new(0)

    for num in arr
        cnt[num] += 1
    end

    cnt = cnt.values.sort

    rm = 0
    ret = 0

    while rm < arr.length / 2
        ret += 1
        rm += cnt.pop
    end

    return ret
end
```
