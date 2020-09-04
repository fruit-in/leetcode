# 1338. 数组大小减半
给你一个整数数组 `arr`。你可以从中选出一个整数集合，并删除这些整数在数组中的每次出现。

返回 **至少** 能删除数组中的一半整数的整数集合的最小大小。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,3,3,3,5,5,5,2,2,7]
<strong>输出:</strong> 2
<strong>解释:</strong> 选择 {3,7} 使得结果数组为 [5,5,5,2,2]、长度为 5（原数组长度的一半）。
大小为 2 的可行集合有 {3,5},{3,2},{5,2}。
选择 {2,7} 是不可行的，它的结果数组为 [3,3,3,3,5,5,5]，新数组长度大于原数组的二分之一。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [7,7,7,7,7,7]
<strong>输出:</strong> 1
<strong>解释:</strong> 我们只能选择集合 {7}，结果数组为空。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,9]
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [1000,1000,3,7]
<strong>输出:</strong> 1
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5,6,7,8,9,10]
<strong>输出:</strong> 5
</pre>

#### 提示:
* `1 <= arr.length <= 10^5`
* `arr.length` 为偶数
* `1 <= arr[i] <= 10^5`

## 题解 (Ruby)

### 1. 计数
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
