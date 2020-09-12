# 1550. 存在连续三个奇数的数组
给你一个整数数组 `arr`，请你判断数组中是否存在连续三个元素都是奇数的情况：如果存在，请返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [2,6,4,1]
<strong>输出:</strong> false
<strong>解释:</strong> 不存在连续三个元素都是奇数的情况。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,2,34,3,4,5,7,23,12]
<strong>输出:</strong> true
<strong>解释:</strong> 存在连续三个元素都是奇数的情况，即 [5,7,23] 。
</pre>

#### 提示:
* `1 <= arr.length <= 1000`
* `1 <= arr[i] <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        for i in range(len(arr) - 2):
            if arr[i] % 2 == arr[i + 1] % 2 == arr[i + 2] % 2 == 1:
                return True

        return False
```
