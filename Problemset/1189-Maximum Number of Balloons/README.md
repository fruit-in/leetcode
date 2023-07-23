# 1189. Maximum Number of Balloons
Given a string ```text```, you want to use the characters of ```text``` to form as many instances of the word **"balloon"** as possible.

You can use each character in ```text``` **at most once**. Return the maximum number of instances that can be formed.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/05/1536_ex1_upd.JPG)
<pre>
<strong>Input:</strong> text = "nlaebolko"
<strong>Output:</strong> 1
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/09/05/1536_ex2_upd.JPG)
<pre>
<strong>Input:</strong> text = "loonbalxballpoon"
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> text = "leetcode"
<strong>Output:</strong> 0
</pre>

#### Constraints:
* ```1 <= text.length <= 10^4```
* ```text``` consists of lower case English letters only.

## Solutions (Rust)

### 1. Count
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
