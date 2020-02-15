# 1331. Rank Transform of an Array
Given an array of integers ```arr```, replace each element with its rank.

The rank represents how large the element is. The rank has the following rules:
* Rank is an integer starting from 1.
* The larger the element, the larger the rank. If two elements are equal, their rank must be the same.
* Rank should be as small as possible.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [40,10,20,30]
<strong>Output:</strong> [4,1,2,3]
<strong>Explanation:</strong> 40 is the largest element. 10 is the smallest. 20 is the second smallest. 30 is the third smallest.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [100,100,100]
<strong>Output:</strong> [1,1,1]
<strong>Explanation:</strong> Same elements share the same rank.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [37,12,28,9,100,56,80,5,12]
<strong>Output:</strong> [5,3,4,2,8,6,7,1,3]
</pre>

#### Constraints:
* <code>0 <= arr.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= arr[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Sort
```Python
class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        sorted_arr = sorted(set(arr))
        rank = {n: i + 1 for i, n in enumerate(sorted_arr)}

        return [rank[n] for n in arr]
```
