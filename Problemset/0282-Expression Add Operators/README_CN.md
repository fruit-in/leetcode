# 282. 给表达式添加运算符
给定一个仅包含数字 `0-9` 的字符串 `num` 和一个目标值整数 `target` ，在 `num` 的数字之间添加 **二元** 运算符（不是一元）`+`、`-` 或 `*` ，返回 **所有** 能够得到 `target` 的表达式。

注意，返回表达式中的操作数 **不应该** 包含前导零。

#### 示例 1:
<pre>
<strong>输入:</strong> num = "123", target = 6
<strong>输出:</strong> ["1*2*3","1+2+3"]
<strong>解释:</strong> “1*2*3” 和 “1+2+3” 的值都是6。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = "232", target = 8
<strong>输出:</strong> ["2*3+2","2+3*2"]
<strong>解释:</strong> “2*3+2” 和 “2+3*2” 的值都是8。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = "3456237490", target = 9191
<strong>输出:</strong> []
<strong>解释:</strong> 表达式 “3456237490” 无法得到 9191 。
</pre>

#### 提示:
* `1 <= num.length <= 10`
* `num` 仅含数字
* <code>-2<sup>31</sup> <= target <= 2<sup>31</sup> - 1</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def addOperators(self, num: str, target: int) -> List[str]:
        def dfs(i: int) -> None:
            if i == len(chars):
                expression = ''.join(chars)
                if pattern.search(expression) is None and eval(expression) == target:
                    ret.append(expression)
                return

            chars[i] = '+'
            dfs(i + 2)
            chars[i] = '-'
            dfs(i + 2)
            chars[i] = '*'
            dfs(i + 2)
            chars[i] = ''
            dfs(i + 2)

        chars = [num[0]]
        pattern = re.compile(r'(?<!\d)0\d+')
        ret = []

        for c in num[1:]:
            chars.append('')
            chars.append(c)

        dfs(1)

        return ret
```
