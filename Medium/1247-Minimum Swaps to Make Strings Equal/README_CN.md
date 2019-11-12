# 1247. 交换字符使得字符串相同
有两个长度相同的字符串 ```s1``` 和 ```s2```，且它们其中 **只含有** 字符 ```"x"``` 和 ```"y"```，你需要通过「交换字符」的方式使这两个字符串相同。

每次「交换字符」的时候，你都可以在两个字符串中各选一个字符进行交换。

交换只能发生在两个不同的字符串之间，绝对不能发生在同一个字符串内部。也就是说，我们可以交换 ```s1[i]``` 和 ```s2[j]```，但不能交换 ```s1[i]``` 和 ```s1[j]```。

最后，请你返回使 ```s1``` 和 ```s2``` 相同的最小交换次数，如果没有方法能够使得这两个字符串相同，则返回 ```-1``` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "xx", s2 = "yy"
<strong>输出:</strong> 1
<strong>解释:</strong>
交换 s1[0] 和 s2[1]，得到 s1 = "yx"，s2 = "yx"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "xy", s2 = "yx"
<strong>输出:</strong> 2
<strong>解释:</strong>
交换 s1[0] 和 s2[0]，得到 s1 = "yy"，s2 = "xx" 。
交换 s1[0] 和 s2[1]，得到 s1 = "xy"，s2 = "xy" 。
注意，你不能交换 s1[0] 和 s1[1] 使得 s1 变成 "yx"，因为我们只能交换属于两个不同字符串的字符。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "xx", s2 = "xy"
<strong>输出:</strong> -1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s1 = "xxyyxyxyxx", s2 = "xyyxyxxxyx"
<strong>输出:</strong> 4
</pre>

#### 提示:
* ```1 <= s1.length, s2.length <= 1000```
* ```s1, s2``` 只包含 ```'x'``` 或 ```'y'```。

## 题解 (Python)

### 1. 题解
```Python3
class Solution:
    def minimumSwap(self, s1: str, s2: str) -> int:
        xy, yx = 0, 0

        for i in range(len(s1)):
            if s1[i] == 'x' and s2[i] == 'y':
                xy += 1
            elif s1[i] == 'y' and s2[i] == 'x':
                yx += 1

        ret = xy // 2 + yx // 2

        if xy % 2 == yx % 2:
            return ret + xy % 2 * 2
        else:
            return -1
```
