# 894. All Possible Full Binary Trees
A *full binary tree* is a binary tree where each node has exactly 0 or 2 children.

Return a list of all possible full binary trees with `N` nodes.  Each element of the answer is the root node of one possible tree.

Each `node` of each tree in the answer **must** have `node.val = 0`.

You may return the final list of trees in any order.

#### Example 1:
<pre>
<b>Input:</b> 7
<b>Output:</b> [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
<b>Explanation:</b>
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/22/fivetrees.png">
</pre>

#### Note:
* `1 <= N <= 20`

## Solutions (Ruby)

### 1. Recursion
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
