# 894. 所有可能的满二叉树
*满二叉树*是一类二叉树，其中每个结点恰好有 0 或 2 个子结点。

返回包含 `N` 个结点的所有可能满二叉树的列表。 答案的每个元素都是一个可能树的根结点。

答案中每个树的每个结点都必须有 `node.val=0`。

你可以按任何顺序返回树的最终列表。

#### 示例:
<pre>
<b>输入:</b> 7
<b>输出:</b> [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
<b>解释:</b>
<img src="https://aliyun-lc-upload.oss-cn-hangzhou.aliyuncs.com/aliyun-lc-upload/uploads/2018/08/24/fivetrees.png">
</pre>

#### 提示:
* `1 <= N <= 20`

## 题解 (Ruby)

### 1. 递归
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
# @param {Integer} n
# @return {TreeNode[]}
def all_possible_fbt(n)
    return [] if n % 2 == 0
    return [TreeNode.new] if n == 1

    ret = []

    for i in (1..(n - 1)).step(2)
        for left in all_possible_fbt(i)
            for right in all_possible_fbt(n - 1 - i)
                ret.push(TreeNode.new(0, left, right))
            end
        end
    end

    return ret
end
```
