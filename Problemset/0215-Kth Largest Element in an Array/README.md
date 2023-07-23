# 215. Kth Largest Element in an Array
Find the **k**th largest element in an unsorted array. Note that it is the kth largest element in the sorted order, not the kth distinct element.

#### Example 1:
<pre>
<b>Input:</b> [3,2,1,5,6,4] and k = 2
<b>Output:</b> 5
</pre>

#### Example 2:
<pre>
<b>Input:</b> [3,2,3,1,2,4,5,5,6] and k = 4
<b>Output:</b> 4
</pre>

#### Note:
You may assume k is always valid, 1 ≤ k ≤ array's length.

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def find_kth_largest(nums, k)
    return nums.sort[-k]
end
```
