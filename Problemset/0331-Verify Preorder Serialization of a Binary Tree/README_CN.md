# 331. 验证二叉树的前序序列化
序列化二叉树的一种方法是使用前序遍历。当我们遇到一个非空节点时，我们可以记录下这个节点的值。如果它是一个空节点，我们可以使用一个标记值记录，例如 `#`。

```
     _9_
    /   \
   3     2
  / \   / \
 4   1  #  6
/ \ / \   / \
# # # #   # #
```

例如，上面的二叉树可以被序列化为字符串 `"9,3,4,#,#,1,#,#,2,#,6,#,#"`，其中 `#` 代表一个空节点。

给定一串以逗号分隔的序列，验证它是否是正确的二叉树的前序序列化。编写一个在不重构树的条件下的可行算法。

每个以逗号分隔的字符或为一个整数或为一个表示 `null` 指针的 `'#'` 。

你可以认为输入格式总是有效的，例如它永远不会包含两个连续的逗号，比如 `"1,,3"` 。

#### 示例 1:
<pre>
<strong>输入:</strong> preorder = "9,3,4,#,#,1,#,#,2,#,6,#,#"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> preorder = "1,#"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> preorder = "9,#,#,1"
<strong>输出:</strong> false
</pre>

## 题解 (Rust)

### 1. 栈
```Rust
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = vec![];

        for x in preorder.split(',') {
            stack.push(x);
            while let Some(&[y, "#", "#"]) = stack.get(stack.len() - 3..stack.len()) {
                if y == "#" {
                    break;
                } else {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push("#");
                }
            }
        }

        &stack == &["#"]
    }
}
```
