# 1189. “气球” 的最大数量
给你一个字符串 ```text```，你需要使用 ```text``` 中的字母来拼凑尽可能多的单词 **"balloon"（气球）**。

字符串 ```text``` 中的每个字母最多只能被使用一次。请你返回最多可以拼凑出多少个单词 **"balloon"**。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/14/1536_ex1_upd.jpeg)
<pre>
<strong>输入:</strong> text = "nlaebolko"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/14/1536_ex2_upd.jpeg)
<pre>
<strong>输入:</strong> text = "loonbalxballpoon"
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text = "leetcode"
<strong>输出:</strong> 0
</pre>

#### 提示:
* ```1 <= text.length <= 10^4```
* ```text``` 全部由小写英文字母组成

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut cnt = [0, 0, 0, 0, 0];
        for ch in text.chars() {
            match ch {
                'b' => cnt[0] += 1,
                'a' => cnt[1] += 1,
                'l' => cnt[2] += 1,
                'o' => cnt[3] += 1,
                'n' => cnt[4] += 1,
                _ => (),
            };
        }
        cnt[2] /= 2;
        cnt[3] /= 2;
        *cnt.iter().min().unwrap()
    }
}
```
