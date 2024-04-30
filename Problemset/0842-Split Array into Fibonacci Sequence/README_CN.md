# 842. 将数组拆分成斐波那契序列
给定一个数字字符串 `num`，比如 `"123456579"`，我们可以将它分成「斐波那契式」的序列 `[123, 456, 579]`。

形式上，**斐波那契式** 序列是一个非负整数列表 `f`，且满足：

* <code>0 <= f[i] < 2<sup>31</sup></code> ，（也就是说，每个整数都符合 **32 位** 有符号整数类型）
* `f.length >= 3`
* 对于所有的`0 <= i < f.length - 2`，都有 `f[i] + f[i + 1] = f[i + 2]`

另外，请注意，将字符串拆分成小块时，每个块的数字一定不要以零开头，除非这个块是数字 `0` 本身。

返回从 `num` 拆分出来的任意一组斐波那契式的序列块，如果不能拆分则返回 `[]`。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "1101111"
<strong>输出:</strong> [11,0,11,11]
<strong>解释:</strong> 输出 [110,1,111] 也可以。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "112358130"
<strong>输出:</strong> []
<strong>解释:</strong> 无法拆分。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "0123"
<strong>输出:</strong> []
<strong>解释:</strong> 每个块的数字不能以零开头，因此 "01"，"2"，"3" 不是有效答案。
</pre>

#### 提示:
* `1 <= num.length <= 200`
* `num` 中只含有数字

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def splitIntoFibonacci(self, num: str) -> List[int]:
        for i in range(1, min(11, (len(num) + 1) // 2)):
            if i > 1 and num[0] == '0':
                break

            for j in range(i + 1, min(i + 11, len(num))):
                if j - i > 1 and num[i] == '0':
                    break

                seq = [int(num[:i]), int(num[i:j])]
                k = j

                while k < len(num):
                    size = len(str(seq[-2] + seq[-1]))
                    seq.append(int(num[k:k + size]))
                    k += size

                    if seq[-1] >= 2147483648 or seq[-3] + seq[-2] != seq[-1]:
                        seq = []
                        break

                if seq != []:
                    return seq

        return []
```
