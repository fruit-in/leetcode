# 1609. 奇偶树
如果一棵二叉树满足下述几个条件，则可以称为 **奇偶树** ：
* 二叉树根节点所在层下标为 `0` ，根的子节点所在层下标为 `1` ，根的孙节点所在层下标为 `2` ，依此类推。
* **偶数下标** 层上的所有节点的值都是 **奇** 整数，从左到右按顺序 **严格递增**
* **奇数下标** 层上的所有节点的值都是 **偶** 整数，从左到右按顺序 **严格递减**

给你二叉树的根节点，如果二叉树为 **奇偶树** ，则返回 `true` ，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_1_1966.png)
<pre>
<strong>输入:</strong> root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
<strong>输出:</strong> true
<strong>解释:</strong> 每一层的节点值分别是：
0 层：[1]
1 层：[10,4]
2 层：[3,7,9]
3 层：[12,8,6,2]
由于 0 层和 2 层上的节点值都是奇数且严格递增，而 1 层和 3 层上的节点值都是偶数且严格递减，因此这是一棵奇偶树。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_2_1966.png)
<pre>
<strong>输入:</strong> root = [5,4,2,3,3,7]
<strong>输出:</strong> false
<strong>解释:</strong> 每一层的节点值分别是：
0 层：[5]
1 层：[4,2]
2 层：[3,3,7]
2 层上的节点值不满足严格递增的条件，所以这不是一棵奇偶树。
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_1_333_1966.png)
<pre>
<strong>输入:</strong> root = [5,9,1,3,5,7]
<strong>输出:</strong> false
<strong>解释:</strong> 1 层上的节点值应为偶数。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> root = [1]
<strong>输出:</strong> true
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> root = [11,8,6,1,3,9,11,30,20,18,16,12,10,4,2,17]
<strong>输出:</strong> true
</pre>

#### 提示:
* 树中节点数在范围 <code>[1, 10<sup>5</sup>]</code> 内
* <code>1 <= Node.val <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 层序遍历
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isEvenOddTree(self, root: TreeNode) -> bool:
        even_index = True
        curr_level = [root]

        while curr_level != []:
            prev_val = 0 if even_index else 1_000_001
            next_level = []

            for node in curr_level:
                if (even_index and
                    (node.val % 2 == 0 or prev_val >= node.val)) or \
                        (not even_index and
                         (node.val % 2 == 1 or prev_val <= node.val)):
                    return False

                prev_val = node.val
                if node.left is not None:
                    next_level.append(node.left)
                if node.right is not None:
                    next_level.append(node.right)

            even_index = not even_index
            curr_level = next_level

        return True
```

## 题解 (Ruby)

### 1. 层序遍历
```Ruby
# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @return {Boolean}
def is_even_odd_tree(root)
  even_index = true
  curr_level = [root]

  until curr_level.empty?
    prev_val = even_index ? 0 : 1_000_001
    next_level = []

    curr_level.each do |node|
      return false if even_index && (node.val.even? || prev_val >= node.val)
      return false if !even_index && (node.val.odd? || prev_val <= node.val)

      prev_val = node.val
      next_level.push(node.left) unless node.left.nil?
      next_level.push(node.right) unless node.right.nil?
    end

    even_index = !even_index
    curr_level = next_level
  end

  true
end
```
