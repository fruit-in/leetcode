# 1306. 跳跃游戏 III
这里有一个非负整数数组 `arr`，你最开始位于该数组的起始下标 `start` 处。当你位于下标 `i` 处时，你可以跳到 `i + arr[i]` 或者 `i - arr[i]`。

请你判断自己是否能够跳到对应元素值为 0 的 **任一** 下标处。

注意，不管是什么情况下，你都无法跳到数组之外。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [4,2,3,0,3,1,2], start = 5
<strong>输出:</strong> true
<strong>解释:</strong>
到达值为 0 的下标 3 有以下可能方案：
下标 5 -> 下标 4 -> 下标 1 -> 下标 3
下标 5 -> 下标 6 -> 下标 4 -> 下标 1 -> 下标 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [4,2,3,0,3,1,2], start = 0
<strong>输出:</strong> true
<strong>解释:</strong>
到达值为 0 的下标 3 有以下可能方案：
下标 0 -> 下标 4 -> 下标 1 -> 下标 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [3,0,2,1,2], start = 2
<strong>输出:</strong> false
<strong>解释:</strong> 无法到达值为 0 的下标 1 处。
</pre>

#### 提示:
* `1 <= arr.length <= 5 * 10^4`
* `0 <= arr[i] < arr.length`
* `0 <= start < arr.length`

## 题解 (Ruby)

### 1. 题解
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
