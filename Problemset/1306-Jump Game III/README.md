# 1306. Jump Game III
Given an array of non-negative integers `arr`, you are initially positioned at `start` index of the array. When you are at index `i`, you can jump to `i + arr[i]` or `i - arr[i]`, check if you can reach to **any** index with value 0.

Notice that you can not jump outside of the array at any time.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [4,2,3,0,3,1,2], start = 5
<strong>Output:</strong> true
<strong>Explanation:</strong>
All possible ways to reach at index 3 with value 0 are:
index 5 -> index 4 -> index 1 -> index 3
index 5 -> index 6 -> index 4 -> index 1 -> index 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [4,2,3,0,3,1,2], start = 0
<strong>Output:</strong> true
<strong>Explanation:</strong>
One possible way to reach at index 3 with value 0 is:
index 0 -> index 4 -> index 1 -> index 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [3,0,2,1,2], start = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no way to reach at index 1 with value 0.
</pre>

#### Constraints:
* `1 <= arr.length <= 5 * 10^4`
* `0 <= arr[i] < arr.length`
* `0 <= start < arr.length`

## Solutions (Ruby)

### 1. Solution
```Ruby
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
```
