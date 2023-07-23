# 1104. 二叉树寻路
在一棵无限的二叉树上，每个节点都有两个子节点，树中的节点 **逐行** 依次按 “之” 字形进行标记。

如下图所示，在奇数行（即，第一行、第三行、第五行……）中，按从左到右的顺序进行标记；

而偶数行（即，第二行、第四行、第六行……）中，按从右到左的顺序进行标记。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/06/28/tree.png)

给你树上某一个节点的标号 ```label```，请你返回从根节点到该标号为 ```label``` 节点的路径，该路径是由途经的节点标号所组成的。

#### 示例 1:
<pre>
<strong>输入:</strong> label = 14
<strong>输出:</strong> [1,3,4,14]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> label = 26
<strong>输出:</strong> [1,2,6,10,26]
</pre>

#### 提示:
* ```1 <= label <= 10^6```

## 题解 (Rust)

### 1. 数学
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
