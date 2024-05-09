# 1178. 猜字谜
外国友人仿照中国字谜设计了一个英文版猜字谜小游戏，请你来猜猜看吧。

字谜的迷面 `puzzle` 按字符串形式给出，如果一个单词 `word` 符合下面两个条件，那么它就可以算作谜底：

* 单词 `word` 中包含谜面 `puzzle` 的第一个字母。
* 单词 `word` 中的每一个字母都可以在谜面 `puzzle` 中找到。
    例如，如果字谜的谜面是 "abcdefg"，那么可以作为谜底的单词有 "faced", "cabbage", 和 "baggage"；而 "beefed"（不含字母 "a"）以及 "based"（其中的 "s" 没有出现在谜面中）都不能作为谜底。

返回一个答案数组 `answer`，数组中的每个元素 `answer[i]` 是在给出的单词列表 `words` 中可以作为字谜迷面 `puzzles[i]` 所对应的谜底的单词数目。

#### 示例 1:
<pre>
<strong>Input:</strong>
words = ["aaaa","asas","able","ability","actt","actor","access"],
puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
<strong>Output:</strong> [1,1,3,2,4,0]
<strong>Explanation:</strong>
1 个单词可以作为 "aboveyz" 的谜底 : "aaaa"
1 个单词可以作为 "abrodyz" 的谜底 : "aaaa"
3 个单词可以作为 "abslute" 的谜底 : "aaaa", "asas", "able"
2 个单词可以作为 "absoryz" 的谜底 : "aaaa", "asas"
4 个单词可以作为 "actresz" 的谜底 : "aaaa", "asas", "actt", "access"
没有单词可以作为 "gaswxyz" 的谜底，因为列表中的单词都不含字母 'g'。
</pre>

#### 提示:
* <code>1 <= words.length <= 10<sup>5</sup></code>
* `4 <= words[i].length <= 50`
* <code>1 <= puzzles.length <= 10<sup>4</sup></code>
* `puzzles[i].length == 7`
* `words[i][j]`, `puzzles[i][j]` 都是小写英文字母。
* 每个 `puzzles[i]` 所包含的字符都不重复。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut answer = vec![0; puzzles.len()];

        for word in words {
            let bits = word.bytes().fold(0_i32, |b, c| b | (1 << (c - b'a')));

            if bits.count_ones() < 8 {
                *count.entry(bits).or_insert(0) += 1;
            }
        }

        for i in 0..answer.len() {
            let puzzle = puzzles[i].as_bytes();
            let mut combins = vec![1 << (puzzle[0] - b'a')];
            answer[i] += count.get(&combins[0]).unwrap_or(&0);

            for j in 1..puzzle.len() {
                for k in 0..combins.len() {
                    let combin = combins[k] | (1 << (puzzle[j] - b'a'));
                    combins.push(combin);
                    answer[i] += count.get(&combin).unwrap_or(&0);
                }
            }
        }

        answer
    }
}
```
