# 925. 长按键入
你的朋友正在使用键盘输入他的名字 ```name```。偶尔，在键入字符 ```c``` 时，按键可能会被*长按*，而字符可能被输入 1 次或多次。

你将会检查键盘输入的字符 ```typed```。如果它对应的可能是你的朋友的名字（其中一些字符可能被长按），那么就返回 ```True```。

#### 示例 1:
<pre>
<strong>输入:</strong> name = "alex", typed = "aaleex"
<strong>输出:</strong> true
<strong>解释:</strong> 'alex' 中的 'a' 和 'e' 被长按。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> name = "saeed", typed = "ssaaedd"
<strong>输出:</strong> false
<strong>解释:</strong> 'e' 一定需要被键入两次，但在 typed 的输出中不是这样。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> name = "leelee", typed = "lleeelee"
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> name = "laiden", typed = "laiden"
<strong>输出:</strong> true
<strong>解释:</strong> 长按名字中的字符并不是必要的。
</pre>

#### 提示:
1. ```name.length <= 1000```
2. ```typed.length <= 1000```
3. ```name``` 和 ```typed``` 的字符都是小写字母。

## 题解 (Python)

### 1. 按块分组
```Python
class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        n_cnt = []
        cnt = 1
        for i in range(len(name)):
            if i < len(name) - 1 and name[i] == name[i + 1]:
                cnt += 1
            else:
                n_cnt.append((name[i], cnt))
                cnt = 1

        t_cnt = []
        cnt = 1
        for i in range(len(typed)):
            if i < len(typed) - 1 and typed[i] == typed[i + 1]:
                cnt += 1
            else:
                t_cnt.append((typed[i], cnt))
                cnt = 1

        return all(n[0] == t[0] and n[1] <= t[1] for n, t in zip(n_cnt, t_cnt)) and len(n_cnt) == len(t_cnt)
```

### 2. 双指针
```Python
class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        i = 0
        for c in typed:
            if i < len(name) and name[i] == c:
                i += 1
            elif i == 0 or name[i - 1] != c:
                return False

        return i == len(name)
```
