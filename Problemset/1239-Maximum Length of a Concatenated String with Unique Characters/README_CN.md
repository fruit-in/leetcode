# 1239. 串联字符串的最大长度
给定一个字符串数组 `arr`，字符串 `s` 是将 `arr` 的含有 **不同字母** 的 **子序列** 字符串 **连接** 所得的字符串。

请返回所有可行解 `s` 中最长长度。

**子序列** 是一种可以从另一个数组派生而来的数组，通过删除某些元素或不删除元素而不改变其余元素的顺序。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = ["un","iq","ue"]
<strong>输出:</strong> 4
<strong>解释:</strong> 所有可能的串联组合是：
- ""
- "un"
- "iq"
- "ue"
- "uniq" ("un" + "iq")
- "ique" ("iq" + "ue")
最大长度为 4。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = ["cha","r","act","ers"]
<strong>输出:</strong> 6
<strong>解释:</strong> 可能的解答有 "chaers" 和 "acters"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = ["abcdefghijklmnopqrstuvwxyz"]
<strong>输出:</strong> 26
</pre>

#### 提示:
* `1 <= arr.length <= 16`
* `1 <= arr[i].length <= 26`
* `arr[i]` 中只含有小写英文字母

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut masks = Vec::new();
        let mut ret = 0;

        for s in &arr {
            let mask = s.bytes().fold(0_i32, |acc, c| acc | (1 << (c - b'a')));

            if mask.count_ones() == s.len() as u32 {
                masks.push(mask);
            }
        }

        for x in 0..2_i32.pow(masks.len() as u32) {
            let mut mask = 0;
            let mut flag = true;

            for i in 0..masks.len() {
                if x & (1 << i) != 0 {
                    if mask ^ masks[i] != mask | masks[i] {
                        flag = false;
                        break;
                    }

                    mask |= masks[i];
                }
            }

            if flag {
                ret = ret.max(mask.count_ones());
            }
        }

        ret as i32
    }
}
```
