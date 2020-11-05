# 430. 扁平化多级双向链表
多级双向链表中，除了指向下一个节点和前一个节点指针之外，它还有一个子链表指针，可能指向单独的双向链表。这些子列表也可能会有一个或多个自己的子项，依此类推，生成多级数据结构，如下面的示例所示。

给你位于列表第一级的头节点，请你扁平化列表，使所有结点出现在单级双链表中。

#### 示例 1:
<pre>
<b>输入:</b> head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
<b>输出:</b> [1,2,3,7,8,11,12,9,10,4,5,6]
<b>解释:</b>
输入的多级列表如下图所示：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/multilevellinkedlist.png">
扁平化后的链表如下图：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/multilevellinkedlistflattened.png">
</pre>

#### 示例 2:
<pre>
<b>输入:</b> head = [1,2,null,3]
<b>输出:</b> [1,3,2]
<b>解释:</b>
输入的多级列表如下图所示：

  1---2---NULL
  |
  3---NULL
</pre>

#### 示例 3:
<pre>
<b>输入:</b> head = []
<b>输出:</b> []
</pre>

#### 如何表示测试用例中的多级链表？
以 **示例 1** 为例：
```
 1---2---3---4---5---6--NULL
         |
         7---8---9---10--NULL
             |
             11--12--NULL
```

序列化其中的每一级之后：
```
[1,2,3,4,5,6,null]
[7,8,9,10,null]
[11,12,null]
```

为了将每一级都序列化到一起，我们需要每一级中添加值为 null 的元素，以表示没有节点连接到上一级的上级节点。
```
[1,2,3,4,5,6,null]
[null,null,7,8,9,10,null]
[null,11,12,null]
```

合并所有序列化结果，并去除末尾的 null 。
```
[1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
```

#### 提示:
* 节点数目不超过 1000
* `1 <= Node.val <= 10^5`

## 题解 (Ruby)

### 1. 深度优先搜索
```Ruby
# Definition for a Node.
# class Node
#     attr_accessor :val, :prev, :next, :child
#     def initialize(val=nil, prev=nil, next_=nil, child=nil)
#         @val = val
#         @prev = prev
#         @next = next_
#         @child = child
#     end
# end

# @param {Node} root
# @return {Node}
def flatten(root)
    return nil if root.nil?

    prev = Node.new
    stack = [root]

    until stack.empty?
        curr = stack.pop

        stack.push(curr.next) unless curr.next.nil?
        stack.push(curr.child) unless curr.child.nil?

        prev.next = curr
        curr.prev = prev
        curr.child = nil

        prev = curr
    end

    root.prev = nil

    return root
end
```
