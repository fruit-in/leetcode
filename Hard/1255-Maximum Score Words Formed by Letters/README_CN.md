# 1255. 得分最高的单词集合
你将会得到一份单词表 `words`，一个字母表 `letters` （可能会有重复字母），以及每个字母对应的得分情况表 `score`。

请你帮忙计算玩家在单词拼写游戏中所能获得的「最高得分」：能够由 `letters` 里的字母拼写出的 **任意** 属于 `words` 单词子集中，分数最高的单词集合的得分。

单词拼写游戏的规则概述如下：

* 玩家需要用字母表 `letters` 里的字母来拼写单词表 `words` 中的单词。
* 可以只使用字母表 `letters` 中的部分字母，但是每个字母最多被使用一次。
* 单词表 `words` 中每个单词只能计分（使用）一次。
* 根据字母得分情况表`score`，字母 `'a'`, `'b'`, `'c'`, ... , `'z'` 对应的得分分别为 `score[0]`, `score[1]`, ..., `score[25]`。
* 本场游戏的「得分」是指：玩家所拼写出的单词集合里包含的所有字母的得分之和。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["dog","cat","dad","good"], letters = ["a","a","c","d","d","d","g","o","o"], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
<strong>输出:</strong> 23
<strong>解释:</strong>
字母得分为  a=1, c=9, d=5, g=3, o=2
使用给定的字母表 letters，我们可以拼写单词 "dad" (5+1+5)和 "good" (3+2+2+5)，得分为 23 。
而单词 "dad" 和 "dog" 只能得到 21 分。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["xxxz","ax","bx","cx"], letters = ["z","a","b","c","x","x","x"], score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
<strong>输出:</strong> 27
<strong>解释:</strong>
字母得分为  a=4, b=4, c=4, x=5, z=10
使用给定的字母表 letters，我们可以组成单词 "ax" (4+5)， "bx" (4+5) 和 "cx" (4+5) ，总得分为 27 。
单词 "xxxz" 的得分仅为 25 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["leetcode"], letters = ["l","e","t","c","o","d"], score = [0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0]
<strong>输出:</strong> 0
<strong>解释:</strong>
字母 "e" 在字母表 letters 中只出现了一次，所以无法组成单词表 words 中的单词。
</pre>

#### 提示:
* `1 <= words.length <= 14`
* `1 <= words[i].length <= 15`
* `1 <= letters.length <= 100`
* `letters[i].length == 1`
* `score.length == 26`
* `0 <= score[i] <= 10`
* `words[i]` 和 `letters[i]` 只包含小写的英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut count0 = [0; 26];
        let mut ret = 0;

        for &c in &letters {
            count0[c as usize - 97] += 1;
        }

        for x in 0..2_i32.pow(words.len() as u32) {
            let mut count1 = count0;
            let mut s = 0;
            let mut flag = false;

            for i in 0..words.len() {
                if flag {
                    break;
                }

                if (1 << i) & x != 0 {
                    for c in words[i].bytes() {
                        if count1[(c - b'a') as usize] <= 0 {
                            flag = true;
                            s = i32::MIN;
                            break;
                        }

                        count1[(c - b'a') as usize] -= 1;
                        s += score[(c - b'a') as usize];
                    }
                }
            }

            ret = ret.max(s);
        }

        ret
    }
}
```
