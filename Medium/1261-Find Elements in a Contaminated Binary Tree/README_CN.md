# 1261. 在受污染的二叉树中查找元素
给出一个满足下述规则的二叉树：
1. ```root.val == 0```
2. 如果 ```treeNode.val == x``` 且 ```treeNode.left != null```，那么 ```treeNode.left.val == 2 * x + 1```
3. 如果 ```treeNode.val == x``` 且 ```treeNode.right != null```，那么 ```treeNode.right.val == 2 * x + 2```

现在这个二叉树受到「污染」，所有的 ```treeNode.val``` 都变成了 ```-1```。

请你先还原二叉树，然后实现 ```FindElements``` 类：
* ```FindElements(TreeNode* root)``` 用受污染的二叉树初始化对象，你需要先把它还原。
* ```bool find(int target)``` 判断目标值 ```target``` 是否存在于还原后的二叉树中并返回结果。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/16/untitled-diagram-4-1.jpg)
<pre>
<strong>输入:</strong>
["FindElements","find","find"]
[[[-1,null,-1]],[1],[2]]
<strong>输出:</strong>
[null,false,true]
<strong>解释:</strong>
FindElements findElements = new FindElements([-1,null,-1]);
findElements.find(1); // return False
findElements.find(2); // return True
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/16/untitled-diagram-4.jpg)
<pre>
<strong>输入:</strong>
["FindElements","find","find","find"]
[[[-1,-1,-1,-1,-1]],[1],[3],[5]]
<strong>输出:</strong>
[null,true,true,false]
<strong>解释:</strong>
FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
findElements.find(1); // return True
findElements.find(3); // return True
findElements.find(5); // return False
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/16/untitled-diagram-4-1-1.jpg)
<pre>
<strong>输入:</strong>
["FindElements","find","find","find","find"]
[[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
<strong>输出:</strong>
[null,true,false,false,true]
<strong>解释:</strong>
FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
findElements.find(2); // return True
findElements.find(3); // return False
findElements.find(4); // return False
findElements.find(5); // return True
</pre>

#### 提示:
* ```TreeNode.val == -1```
* 二叉树的高度不超过 ```20```
* 节点的总数在 ```[1, 10^4]``` 之间
* 调用 ```find()``` 的总次数在 ```[1, 10^4]``` 之间
* ```0 <= target <= 10^6```

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class FindElements:

    def __init__(self, root: TreeNode):
        self.vals = set()
        root.val = 0
        stack = [root]

        while stack:
            curr = stack.pop()
            self.vals.add(curr.val)

            if curr.left:
                curr.left.val = 2 * curr.val + 1
                stack.append(curr.left)
            if curr.right:
                curr.right.val = 2 * curr.val + 2
                stack.append(curr.right)

    def find(self, target: int) -> bool:
        return target in self.vals


# Your FindElements object will be instantiated and called as such:
# obj = FindElements(root)
# param_1 = obj.find(target)
```
