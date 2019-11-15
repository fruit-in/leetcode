# 443. 压缩字符串
给定一组字符，使用[原地算法](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)将其压缩。

压缩后的长度必须始终小于或等于原数组长度。

数组的每个元素应该是长度为1 的**字符**（不是 int 整数类型）。

在完成[原地](https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95)**修改输入数组**后，返回数组的新长度。

#### 进阶:
你能否仅使用O(1) 空间解决问题？

#### 示例 1:
<pre>
<strong>输入:</strong>
["a","a","b","b","c","c","c"]
<strong>输出:</strong>
返回6，输入数组的前6个字符应该是：["a","2","b","2","c","3"]
<strong>说明:</strong>
"aa"被"a2"替代。"bb"被"b2"替代。"ccc"被"c3"替代。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
["a"]
<strong>输出:</strong>
返回1，输入数组的前1个字符应该是：["a"]
<strong>说明:</strong>
没有任何字符串被替代。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong>
["a","b","b","b","b","b","b","b","b","b","b","b","b"]
<strong>输出:</strong>
返回4，输入数组的前4个字符应该是：["a","b","1","2"]。
<strong>说明:</strong>
由于字符"a"不重复，所以不会被压缩。"bbbbbbbbbbbb"被“b12”替代。
注意每个数字在数组中都有它自己的位置。
</pre>

#### 注意:
1. 所有字符都有一个ASCII值在```[35, 126]```区间内。
2. ```1 <= len(chars) <= 1000```.

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def compress(self, chars: List[str]) -> int:
        r, w_char, w_pos = 0, 0, 0

        for r in range(len(chars)):
            if r == len(chars) - 1 or chars[r] != chars[r + 1]:
                s = chars[w_char] + (str(r - w_char + 1) if r > w_char else "")

                for ch in s:
                    chars[w_pos] = ch
                    w_pos += 1

                w_char = r + 1

        return w_pos
```
