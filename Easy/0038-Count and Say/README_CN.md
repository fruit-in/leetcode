# 38. 报数
报数序列是一个整数序列，按照其中的整数的顺序进行报数，得到下一个数。其前五项如下：

```
1.     1
2.     11
3.     21
4.     1211
5.     111221
```

```1``` 被读作  ```"one 1"```  (```"一个一"```) , 即 ```11```。

```11``` 被读作 ```"two 1s"``` (```"两个一"```）, 即 ```21```。

```21``` 被读作 ```"one 2"```,  ```"one 1"``` （```"一个二"``` ,  ```"一个一"```) , 即 ```1211```。

给定一个正整数 *n*（1 ≤ *n* ≤ 30），输出报数序列的第 *n* 项。

注意：整数顺序将表示为一个字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> "1"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> "1211"
</pre>

## 题解 (Python)

### 1. 双指针
```Python3
class Solution:
    def countAndSay(self, n: int) -> str:
        s = "1"
        for i in range(n - 1):
            tmp = ""
            i = 0
            for j in range(len(s)):
                if s[i] != s[j]:
                    tmp += str(j - i) + s[i]
                    i = j
            tmp += str(len(s) - i) + s[i]
            s = tmp
        return s
```
