# 1104. Path In Zigzag Labelled Binary Tree
In an infinite binary tree where every node has two children, the nodes are labelled in row order.

In the odd numbered rows (ie., the first, third, fifth,...), the labelling is left to right, while in the even numbered rows (second, fourth, sixth,...), the labelling is right to left.

![](https://assets.leetcode.com/uploads/2019/06/24/tree.png)

Given the ```label``` of a node in this tree, return the labels in the path from the root of the tree to the node with that ```label```.

#### Example 1:
<pre>
<strong>Input:</strong> label = 14
<strong>Output:</strong> [1,3,4,14]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> label = 26
<strong>Output:</strong> [1,2,6,10,26]
</pre>

#### Constraints:
* ```1 <= label <= 10^6```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut x = 2_i32.pow((label as f64).log2() as u32);
        let mut label = label;
        let mut ret = vec![label];

        while label > 1 {
            label = x - 1 - ((label / 2) % (x / 2));
            x /= 2;
            ret.push(label);
        }

        ret.reverse();
        ret
    }
}
```
