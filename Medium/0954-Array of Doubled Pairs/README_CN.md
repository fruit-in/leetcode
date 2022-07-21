# 954. 二倍数对数组
给定一个长度为偶数的整数数组 `arr`，只有对 `arr` 进行重组后可以满足 “对于每个 `0 <= i < len(arr) / 2`，都有 `arr[2 * i + 1] = 2 * arr[2 * i]`” 时，返回 `true`；否则，返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [3,1,3,6]
<strong>输出:</strong> false
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,1,2,6]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [4,-2,2,-4]
<strong>输出:</strong> true
<strong>解释:</strong> 可以用 [-2,-4] 和 [2,4] 这两组组成 [-2,-4,2,4] 或是 [2,4,-2,-4]
</pre>

#### 提示:
* <code>2 <= arr.length <= 3 * 10<sup>4</sup></code>
* `arr.length` 是偶数
* <code>-10<sup>5</sup> <= arr[i] <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def canReorderDoubled(self, arr: List[int]) -> bool:
        count = {}
        nums = []

        for x in arr:
            if x % 2 == 0:
                if x not in count:
                    count[x] = 0
                count[x] += 1
            else:
                if 2 * x not in count:
                    count[2 * x] = 0
                count[2 * x] -= 1

        for x, v in count.items():
            if v < 0:
                return False
            elif v > 0:
                nums.append(x)

        nums.sort(key=lambda x: (x >= 0, abs(x)))

        for x in nums:
            if count[x] == 0:
                continue
            elif 2 * x in count and count[x] <= count[2 * x]:
                count[2 * x] -= count[x]
                count[x] = 0
            else:
                return False

        return True
```
