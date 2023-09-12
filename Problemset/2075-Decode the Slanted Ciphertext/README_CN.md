# 2075. 解码斜向换位密码
字符串 `originalText` 使用 **斜向换位密码** ，经由 **行数固定** 为 `rows` 的矩阵辅助，加密得到一个字符串 `encodedText` 。

`originalText` 先按从左上到右下的方式放置到矩阵中。

![](https://assets.leetcode.com/uploads/2021/11/07/exa11.png)

先填充蓝色单元格，接着是红色单元格，然后是黄色单元格，以此类推，直到到达 `originalText` 末尾。箭头指示顺序即为单元格填充顺序。所有空单元格用 `' '` 进行填充。矩阵的列数需满足：用 `originalText` 填充之后，最右侧列 **不为空** 。

接着按行将字符附加到矩阵中，构造 `encodedText` 。

![](https://assets.leetcode.com/uploads/2021/11/07/exa12.png)

先把蓝色单元格中的字符附加到 `encodedText` 中，接着是红色单元格，最后是黄色单元格。箭头指示单元格访问顺序。

例如，如果 `originalText = "cipher"` 且 `rows = 3` ，那么我们可以按下述方法将其编码：

![](https://assets.leetcode.com/uploads/2021/10/25/desc2.png)

蓝色箭头标识 `originalText` 是如何放入矩阵中的，红色箭头标识形成 `encodedText` 的顺序。在上述例子中，`encodedText = "ch   ie   pr"` 。

给你编码后的字符串 `encodedText` 和矩阵的行数 `rows` ，返回源字符串 `originalText` 。

**注意：**`originalText` **不** 含任何尾随空格 `' '` 。生成的测试用例满足 **仅存在一个** 可能的 `originalText` 。

#### 示例 1:
<pre>
<strong>输入:</strong> encodedText = "ch   ie   pr", rows = 3
<strong>输出:</strong> "cipher"
<strong>解释:</strong> 此示例与问题描述中的例子相同。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/26/exam1.png)
<pre>
<strong>输入:</strong> encodedText = "iveo    eed   l te   olc", rows = 4
<strong>输出:</strong> "i love leetcode"
<strong>解释:</strong> 上图标识用于编码 originalText 的矩阵。
蓝色箭头展示如何从 encodedText 找到 originalText 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/10/26/eg2.png)
<pre>
<strong>输入:</strong> encodedText = "coding", rows = 1
<strong>输出:</strong> "coding"
<strong>解释:</strong> 由于只有 1 行，所以 originalText 和 encodedText 是相同的。
</pre>

#### 示例 4:
![](https://assets.leetcode.com/uploads/2021/10/26/exam3.png)
<pre>
<strong>输入:</strong> encodedText = " b  ac", rows = 2
<strong>输出:</strong> " abc"
<strong>解释:</strong> originalText 不能含尾随空格，但它可能会有一个或者多个前置空格。
</pre>

#### 提示:
* <code>0 <= encodedText.length <= 10<sup>6</sup></code>
* `encodedText` 仅由小写英文字母和 `' '` 组成
* `encodedText` 是对某个 **不含** 尾随空格的 `originalText` 的一个有效编码
* `1 <= rows <= 1000`
* 生成的测试用例满足 **仅存在一个** 可能的 `originalText`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let cols = encoded_text.len() / rows;
        let encoded_text = encoded_text.as_bytes();
        let mut original_text = vec![];

        for col in 0..cols {
            for row in 0..rows {
                if col + row >= cols {
                    break;
                }

                original_text.push(encoded_text[row * cols + col + row]);
            }
        }

        String::from_utf8(original_text)
            .unwrap()
            .trim_end()
            .to_string()
    }
}
```
