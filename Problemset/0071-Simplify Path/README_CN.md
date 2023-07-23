# 71. 简化路径
以 Unix 风格给出一个文件的**绝对路径**，你需要简化它。或者换句话说，将其转换为规范路径。

在 Unix 风格的文件系统中，一个点（`.`）表示当前目录本身；此外，两个点 （`..`） 表示将目录切换到上一级（指向父目录）；两者都可以是复杂相对路径的组成部分。更多信息请参阅：[Linux / Unix中的绝对路径 vs 相对路径](https://blog.csdn.net/u011327334/article/details/50355600)

请注意，返回的规范路径必须始终以斜杠 `/` 开头，并且两个目录名之间必须只有一个斜杠 `/`。最后一个目录名（如果存在）**不能**以 `/` 结尾。此外，规范路径必须是表示绝对路径的**最短**字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> "/home/"
<strong>输出:</strong> "/home"
<strong>解释:</strong> 注意，最后一个目录名后面没有斜杠。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "/../"
<strong>输出:</strong> "/"
<strong>解释:</strong> 从根目录向上一级是不可行的，因为根是你可以到达的最高级。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "/home//foo/"
<strong>输出:</strong> "/home/foo"
<strong>解释:</strong> 在规范路径中，多个连续斜杠需要用一个斜杠替换。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "/a/./b/../../c/"
<strong>输出:</strong> "/c"
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> "/a/../../b/../c//.//"
<strong>输出:</strong> "/c"
</pre>

#### 示例 6:
<pre>
<strong>输入:</strong> "/a//b////c/d//././/.."
<strong>输出:</strong> "/a/b/c"
</pre>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def simplifyPath(self, path: str) -> str:
        stack = []

        for d in path.split('/'):
            if d == '..':
                if stack:
                    stack.pop()
            elif d != '.' and d != '':
                stack.append(d)

        return '/' + '/'.join(stack)
```
