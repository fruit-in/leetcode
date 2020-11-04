# 215. 数组中的第K个最大元素
在未排序的数组中找到第 **k** 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

#### 示例 1:
<pre>
<b>输入:</b> [3,2,1,5,6,4] and k = 2
<b>输出:</b> 5
</pre>

#### 示例 2:
<pre>
<b>输入:</b> [3,2,3,1,2,4,5,5,6] and k = 4
<b>输出:</b> 4
</pre>

#### 说明:
你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def find_kth_largest(nums, k)
    return nums.sort[-k]
end
```
