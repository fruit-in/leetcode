# 1095. Find in Mountain Array
*(This problem is an **interactive problem**.)*

You may recall that an array `arr` is a **mountain array** if and only if:
* `arr.length >= 3`
* There exists some `i` with `0 < i < arr.length - 1` such that:
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

Given a mountain array `mountainArr`, return the **minimum** `index` such that `mountainArr.get(index) == target`. If such an `index` does not exist, return `-1`.

**You cannot access the mountain array directly**. You may only access the array using a `MountainArray` interface:
* `MountainArray.get(k)` returns the element of the array at index `k` (0-indexed).
* `MountainArray.length()` returns the length of the array.

Submissions making more than `100` calls to `MountainArray.get` will be judged *Wrong Answer*. Also, any solutions that attempt to circumvent the judge will result in disqualification.

#### Example 1:
<pre>
<strong>Input:</strong> mountainArr = [1,2,3,4,5,3,1], target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> 3 exists in the array, at index=2 and index=5. Return the minimum index, which is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> mountainArr = [0,1,2,4,2,1], target = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> 3 does not exist in the array, so we return -1.
</pre>

#### Constraints:
* <code>3 <= mountainArr.length() <= 10<sup>4</sup></code>
* <code>0 <= target <= 10<sup>9</sup></code>
* <code>0 <= mountainArr.get(index) <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
# """
# This is MountainArray's API interface.
# You should not implement it, or speculate about its implementation
# """
# class MountainArray:
#    def get(self, index: int) -> int:
#    def length(self) -> int:

class Solution:
    def findInMountainArray(self, target: int, mountainArr: 'MountainArray') -> int:
        n = mountainArr.length()
        i = bisect.bisect(range(n - 1), False, key=lambda j: mountainArr.get(
            j) > mountainArr.get(j + 1))

        index = bisect.bisect_left(range(n), target, hi=i, key=mountainArr.get)
        if mountainArr.get(index) == target:
            return index
        index = bisect.bisect_left(
            range(n - 1), -target, lo=i, key=lambda j: -mountainArr.get(j))
        if mountainArr.get(index) == target:
            return index

        return -1
```
