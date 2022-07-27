# 1577. Number of Ways Where Square of Number Is Equal to Product of Two Numbers
Given two arrays of integers `nums1` and `nums2`, return the number of triplets formed (type 1 and type 2) under the following rules:
* Type 1: Triplet (i, j, k) if <code>nums1[i]<sup>2</sup> == nums2[j] * nums2[k]</code> where `0 <= i < nums1.length` and `0 <= j < k < nums2.length`.
* Type 2: Triplet (i, j, k) if <code>nums2[i]<sup>2</sup> == nums1[j] * nums1[k]</code> where `0 <= i < nums2.length` and `0 <= j < k < nums1.length`.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [7,4], nums2 = [5,2,8,9]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Type 1: (1, 1, 2), nums1[1]<sup>2</sup> = nums2[1] * nums2[2]. (4<sup>2</sup> = 2 * 8).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,1], nums2 = [1,1,1]
<strong>Output:</strong> 9
<strong>Explanation:</strong> All Triplets are valid, because 1<sup>2</sup> = 1 * 1.
Type 1: (0,0,1), (0,0,2), (0,1,2), (1,0,1), (1,0,2), (1,1,2).  nums1[i]<sup>2</sup> = nums2[j] * nums2[k].
Type 2: (0,0,1), (1,0,1), (2,0,1). nums2[i]<sup>2</sup> = nums1[j] * nums1[k].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [7,7,8,3], nums2 = [1,2,9,7]
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 valid triplets.
Type 1: (3,0,2).  nums1[3]<sup>2</sup> = nums2[0] * nums2[2].
Type 2: (3,0,1).  nums2[3]<sup>2</sup> = nums1[0] * nums1[1].
</pre>

#### Constraints:
* `1 <= nums1.length, nums2.length <= 1000`
* <code>1 <= nums1[i], nums2[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numTriplets(self, nums1: List[int], nums2: List[int]) -> int:
        count1 = Counter(nums1)
        count2 = Counter(nums2)
        ret = 0

        for x in nums1:
            x2 = x * x
            for y in nums2:
                if x2 % y == 0 and x2 // y in count2:
                    ret += count2[x2 // y]
                    if y == x:
                        ret -= 1
        for x in nums2:
            x2 = x * x
            for y in nums1:
                if x2 % y == 0 and x2 // y in count1:
                    ret += count1[x2 // y]
                    if y == x:
                        ret -= 1

        return ret // 2
```
