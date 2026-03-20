# 385. 迷你语法分析器
给定一个字符串 s 表示一个整数嵌套列表，实现一个解析它的语法分析器并返回解析的结果 `NestedInteger` 。

列表中的每个元素只可能是整数或整数嵌套列表

#### 示例 1:
<pre>
<strong>输入:</strong> s = "324"
<strong>输出:</strong> 324
<strong>解释:</strong> 你应该返回一个 NestedInteger 对象，其中只包含整数值 324。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "[123,[456,[789]]]"
<strong>输出:</strong> [123,[456,[789]]]
<strong>解释:</strong> 返回一个 NestedInteger 对象包含一个有两个元素的嵌套列表：
1. 一个 integer 包含值 123
2. 一个包含两个元素的嵌套列表：
    i.  一个 integer 包含值 456
    ii. 一个包含一个元素的嵌套列表
         a. 一个 integer 包含值 789
</pre>

#### 提示:
* <code>1 <= s.length <= 5 * 10<sup>4</sup></code>
* `s` 由数字、方括号 `"[]"`、负号 `'-'` 、逗号 `','`组成
* 用例保证 `s` 是可解析的 `NestedInteger`
* 输入中的所有值的范围是 <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code>

## 题解 (Python)

### 1. 题解
```Python
# """
# This is the interface that allows for creating nested lists.
# You should not implement it, or speculate about its implementation
# """
# class NestedInteger:
#    def __init__(self, value=None):
#        """
#        If value is not specified, initializes an empty list.
#        Otherwise initializes a single integer equal to value.
#        """
#
#    def isInteger(self):
#        """
#        @return True if this NestedInteger holds a single integer, rather than a nested list.
#        :rtype bool
#        """
#
#    def add(self, elem):
#        """
#        Set this NestedInteger to hold a nested list and adds a nested integer elem to it.
#        :rtype void
#        """
#
#    def setInteger(self, value):
#        """
#        Set this NestedInteger to hold a single integer equal to value.
#        :rtype void
#        """
#
#    def getInteger(self):
#        """
#        @return the single integer that this NestedInteger holds, if it holds a single integer
#        Return None if this NestedInteger holds a nested list
#        :rtype int
#        """
#
#    def getList(self):
#        """
#        @return the nested list that this NestedInteger holds, if it holds a nested list
#        Return None if this NestedInteger holds a single integer
#        :rtype List[NestedInteger]
#        """

class Solution:
    def deserialize(self, s: str) -> NestedInteger:
        if s[0] != '[':
            return NestedInteger(int(s))

        stack = [NestedInteger()]
        neg = False
        num = 0

        for i, c in enumerate(s):
            if c == '[':
                stack.append(NestedInteger())
                stack[-2].add(stack[-1])
            elif c == ']' or c == ',':
                if s[i - 1].isdigit():
                    if neg:
                        num = -num
                    stack[-1].add(NestedInteger(num))
                    neg = False
                    num = 0
                if c == ']':
                    stack.pop()
            elif c == '-':
                neg = True
            else:
                num = num * 10 + int(c)

        return stack[0].getList()[0]
```
