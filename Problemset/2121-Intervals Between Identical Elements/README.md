# 2121. Intervals Between Identical Elements
You are given a **0-indexed** array of `n` integers `arr`.

The **interval** between two elements in `arr` is defined as the **absolute difference** between their indices. More formally, the **interval** between `arr[i]` and `arr[j]` is `|i - j|`.

Return *an array* `intervals` *of length* `n` *where* `intervals[i]` *is **the sum of intervals** between* `arr[i]` *and each element in* `arr` *with the same value as* `arr[i]`.

**Note:** `|x|` is the absolute value of `x`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,1,3,1,2,3,3]
<strong>Output:</strong> [4,2,7,2,4,4,5]
<strong>Explanation:</strong>
- Index 0: Another 2 is found at index 4. |0 - 4| = 4
- Index 1: Another 1 is found at index 3. |1 - 3| = 2
- Index 2: Two more 3s are found at indices 5 and 6. |2 - 5| + |2 - 6| = 7
- Index 3: Another 1 is found at index 1. |3 - 1| = 2
- Index 4: Another 2 is found at index 0. |4 - 0| = 4
- Index 5: Two more 3s are found at indices 2 and 6. |5 - 2| + |5 - 6| = 4
- Index 6: Two more 3s are found at indices 2 and 5. |6 - 2| + |6 - 5| = 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [10,5,10,10]
<strong>Output:</strong> [5,0,3,4]
<strong>Explanation:</strong>
- Index 0: Two more 10s are found at indices 2 and 3. |0 - 2| + |0 - 3| = 5
- Index 1: There is only one 5 in the array, so its sum of intervals to identical elements is 0.
- Index 2: Two more 10s are found at indices 0 and 3. |2 - 0| + |2 - 3| = 3
- Index 3: Two more 10s are found at indices 0 and 2. |3 - 0| + |3 - 2| = 4
</pre>

#### Constraints:
* `n == arr.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def getDistances(self, arr: List[int]) -> List[int]:
        indices_sums = {}
        indices = [0] * len(arr)
        intervals = [0] * len(arr)

        for i in range(len(arr)):
            if arr[i] in indices_sums:
                indices_sums[arr[i]].append(i + indices_sums[arr[i]][-1])
            else:
                indices_sums[arr[i]] = [i]
            indices[i] = len(indices_sums[arr[i]]) - 1

        for i in range(len(arr)):
            prefix_sum = indices_sums[arr[i]]
            intervals[i] = prefix_sum[-1] + 2 * i * indices[i] + 2 * i \
                - 2 * prefix_sum[indices[i]] - i * len(prefix_sum)

        return intervals
```
