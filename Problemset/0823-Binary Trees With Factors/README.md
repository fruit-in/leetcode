# 823. Binary Trees With Factors
Given an array of unique integers, `arr`, where each integer `arr[i]` is strictly greater than `1`.

We make a binary tree using these integers, and each number may be used for any number of times. Each non-leaf node's value should be equal to the product of the values of its children.

Return *the number of binary trees we can make*. The answer may be too large so return the answer **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,4]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can make these trees: [2], [4], [4, 2, 2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,4,5,10]
<strong>Output:</strong> 7
<strong>Explanation:</strong> We can make these trees: [2], [4], [5], [10], [4, 2, 2], [10, 2, 5], [10, 5, 2].
</pre>

#### Constraints:
* `1 <= arr.length <= 1000`
* <code>2 <= arr[i] <= 10<sup>9</sup></code>
* All the values of `arr` are **unique**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numFactoredBinaryTrees(self, arr: List[int]) -> int:
        count = {}
        ret = 0

        arr.sort()

        for i in range(len(arr)):
            count[arr[i]] = 1

            for j in range(i):
                if arr[i] % arr[j] == 0:
                    count[arr[i]] = (
                        count[arr[i]] + count[arr[j]] * count.get(arr[i] // arr[j], 0)) % 1_000_000_007

            ret = (ret + count[arr[i]]) % 1_000_000_007

        return ret
```
