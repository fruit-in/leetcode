# 385. Mini Parser
Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return *the deserialized* `NestedInteger`.

Each element is either an integer or a list whose elements may also be integers or other lists.

#### Example 1:
<pre>
<strong>Input:</strong> s = "324"
<strong>Output:</strong> 324
<strong>Explanation:</strong> You should return a NestedInteger object which contains a single integer 324.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "[123,[456,[789]]]"
<strong>Output:</strong> [123,[456,[789]]]
<strong>Explanation:</strong> Return a NestedInteger object containing a nested list with 2 elements:
1. An integer containing value 123.
2. A nested list containing two elements:
    i.  An integer containing value 456.
    ii. A nested list with one element:
         a. An integer containing value 789
</pre>

#### Constraints:
* <code>1 <= s.length <= 5 * 10<sup>4</sup></code>
* `s` consists of digits, square brackets `"[]"`, negative sign `'-'`, and commas `','`.
* `s` is the serialization of valid `NestedInteger`.
* All the values in the input are in the range <code>[-10<sup>6</sup>, 10<sup>6</sup>]</code>.

## Solutions (Python)

### 1. Solution
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
