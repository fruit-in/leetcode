# 68. 文本左右对齐
给定一个单词数组 `words` 和一个长度 `maxWidth` ，重新排版单词，使其成为每行恰好有 `maxWidth` 个字符，且左右两端对齐的文本。

你应该使用 **“贪心算法”** 来放置给定的单词；也就是说，尽可能多地往每行中放置单词。必要时可用空格 `' '` 填充，使得每行恰好有 *maxWidth* 个字符。

要求尽可能均匀分配单词间的空格数量。如果某一行单词间的空格不能均匀分配，则左侧放置的空格数要多于右侧的空格数。

文本的最后一行应为左对齐，且单词之间不插入**额外的**空格。

#### 注意:
* 单词是指由非空格字符组成的字符序列。
* 每个单词的长度大于 0，小于等于 *maxWidth*。
* 输入单词数组 `words` 至少包含一个单词。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
<strong>输出:</strong>
[
   "This    is    an",
   "example  of text",
   "justification.  "
]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
<strong>输出:</strong>
[
  "What   must   be",
  "acknowledgment  ",
  "shall be        "
]
<strong>解释:</strong> 注意最后一行的格式应为 "shall be    " 而不是 "shall     be",
     因为最后一行应为左对齐，而不是左右两端对齐。
     第二行同样为左对齐，这是因为这行只包含一个单词。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
<strong>输出:</strong>
[
  "Science  is  what we",
  "understand      well",
  "enough to explain to",
  "a  computer.  Art is",
  "everything  else  we",
  "do                  "
]
</pre>

#### 提示:
* `1 <= words.length <= 300`
* `1 <= words[i].length <= 20`
* `words[i]` 由小写英文字母和符号组成
* `1 <= maxWidth <= 100`
* `words[i].length <= maxWidth`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut last_len = max_width;
        let mut lines = vec![];
        let mut ret = vec![];

        for word in words {
            if last_len + 1 + word.len() > max_width {
                last_len = word.len();
                lines.push(vec![word]);
            } else {
                last_len += 1 + word.len();
                lines.last_mut().unwrap().push(word);
            }
        }

        for i in 0..lines.len() {
            let words_len = lines[i].iter().map(|w| w.len()).sum::<usize>();
            let mut spaces_len = (max_width - words_len) / (lines[i].len().max(2) - 1);
            let mut left_len = (max_width - words_len) % (lines[i].len().max(2) - 1);
            let mut line = vec![];

            if i == lines.len() - 1 {
                spaces_len = 1;
                left_len = 0;
            }

            for j in 0..lines[i].len() {
                line.push(lines[i][j].clone());
                if j < lines[i].len() - 1 {
                    if left_len > 0 {
                        line.push(" ".repeat(spaces_len + 1));
                        left_len -= 1;
                    } else {
                        line.push(" ".repeat(spaces_len));
                    }
                }
            }

            let mut line = line.concat();

            if line.len() < max_width {
                line += &" ".repeat(max_width - line.len());
            }

            ret.push(line);
        }

        ret
    }
}
```
