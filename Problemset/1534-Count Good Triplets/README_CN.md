# 1534. 统计好三元组
给你一个整数数组 `arr` ，以及 `a`、`b` 、`c` 三个整数。请你统计其中好三元组的数量。

如果三元组 `(arr[i], arr[j], arr[k])` 满足下列全部条件，则认为它是一个 **好三元组** 。
* `0 <= i < j < k < arr.length`
* `|arr[i] - arr[j]| <= a`
* `|arr[j] - arr[k]| <= b`
* `|arr[i] - arr[k]| <= c`

其中 `|x|` 表示 `x` 的绝对值。

返回 **好三元组的数量** 。

### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,0,1,1,9,7], a = 7, b = 2, c = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 一共有 4 个好三元组：[(3,0,1), (3,0,1), (3,1,1), (0,1,1)] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,2,2,3], a = 0, b = 0, c = 1
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在满足所有条件的三元组。
</pre>

#### 提示:
* `3 <= arr.length <= 100`
* `0 <= arr[i] <= 1000`
* `0 <= a, b, c <= 1000`

## 题解 (Ruby)

### 1. 暴力
```Ruby
# @param {Integer[]} arr
# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def count_good_triplets(arr, a, b, c)
    ret = 0

    for i in 0...arr.length
        for j in (i + 1)...arr.length
            if (arr[i] - arr[j]).abs <= a
                for k in (j + 1)...arr.length
                    if (arr[j] - arr[k]).abs <= b and (arr[i] - arr[k]).abs <= c
                        ret += 1
                    end
                end
            end
        end
    end

    return ret
end
```
