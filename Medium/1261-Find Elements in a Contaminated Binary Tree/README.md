# 1261. Find Elements in a Contaminated Binary Tree
Given a binary tree with the following rules:
1. ```root.val == 0```
2. If ```treeNode.val == x``` and ```treeNode.left != null```, then ```treeNode.left.val == 2 * x + 1```
3. If ```treeNode.val == x``` and ```treeNode.right != null```, then ```treeNode.right.val == 2 * x + 2```

Now the binary tree is contaminated, which means all ```treeNode.val``` have been changed to ```-1```.

You need to first recover the binary tree and then implement the ```FindElements``` class:
* ```FindElements(TreeNode* root)``` Initializes the object with a contamined binary tree, you need to recover it first.
* ```bool find(int target)``` Return if the ```target``` value exists in the recovered binary tree.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4-1.jpg)
<pre>
<strong>Input:</strong>
["FindElements","find","find"]
[[[-1,null,-1]],[1],[2]]
<strong>Output:</strong>
[null,false,true]
<strong>Explanation:</strong>
FindElements findElements = new FindElements([-1,null,-1]);
findElements.find(1); // return False
findElements.find(2); // return True
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/11/06/untitled-diagram-4.jpg)
<pre>
<strong>Input:</strong>
["FindElements","find","find","find"]
[[[-1,-1,-1,-1,-1]],[1],[3],[5]]
<strong>Output:</strong>
[null,true,true,false]
<strong>Explanation:</strong>
FindElements findElements = new FindElements([-1,-1,-1,-1,-1]);
findElements.find(1); // return True
findElements.find(3); // return True
findElements.find(5); // return False
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/11/07/untitled-diagram-4-1-1.jpg)
<pre>
<strong>Input:</strong>
["FindElements","find","find","find","find"]
[[[-1,null,-1,-1,null,-1]],[2],[3],[4],[5]]
<strong>Output:</strong>
[null,true,false,false,true]
<strong>Explanation:</strong>
FindElements findElements = new FindElements([-1,null,-1,-1,null,-1]);
findElements.find(2); // return True
findElements.find(3); // return False
findElements.find(4); // return False
findElements.find(5); // return True
</pre>

#### Constraints:
* ```TreeNode.val == -1```
* The height of the binary tree is less than or equal to ```20```
* The total number of nodes is between ```[1, 10^4]```
* Total calls of ```find()``` is between ```[1, 10^4]```
* ```0 <= target <= 10^6```

## Solutions (Python)

### 1. Solution
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
