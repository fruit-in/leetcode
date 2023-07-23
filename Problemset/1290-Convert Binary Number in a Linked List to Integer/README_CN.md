# 1290. 二进制链表转整数
给你一个单链表的引用结点 ```head```。链表中每个结点的值不是 0 就是 1。已知此链表是一个整数数字的二进制表示形式。

请你返回该链表所表示数字的 **十进制值** 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/12/15/graph-1.png)
<pre>
<strong>输入:</strong> head = [1,0,1]
<strong>输出:</strong> 5
<strong>解释:</strong> 二进制数 (101) 转化为十进制数 (5)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> head = [0]
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> head = [1]
<strong>输出:</strong> 1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> head = [1,0,0,1,0,0,1,1,1,0,0,0,0,0,0]
<strong>输出:</strong> 18880
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> head = [0,0]
<strong>输出:</strong> 0
</pre>

#### 提示:
* 链表不为空。
* 链表的结点总数不超过 ```30```。
* 每个结点的值不是 ```0``` 就是 ```1```。

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getDecimalValue(self, head: ListNode) -> int:
        ret = 0

        while head:
            ret <<= 1
            ret += head.val
            head = head.next

        return ret
```
