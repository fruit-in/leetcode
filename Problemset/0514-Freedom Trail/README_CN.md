# 514. 自由之路
电子游戏“辐射4”中，任务 **“通向自由”** 要求玩家到达名为 **“Freedom Trail Ring”** 的金属表盘，并使用表盘拼写特定关键词才能开门。

给定一个字符串 `ring` ，表示刻在外环上的编码；给定另一个字符串 `key` ，表示需要拼写的关键词。您需要算出能够拼写关键词中所有字符的**最少**步数。

最初，**ring** 的第一个字符与 `12:00` 方向对齐。您需要顺时针或逆时针旋转 `ring` 以使 **key** 的一个字符在 `12:00` 方向对齐，然后按下中心按钮，以此逐个拼写完 `key` 中的所有字符。

旋转 `ring` 拼出 key 字符 `key[i]` 的阶段中：

1. 您可以将 **ring** 顺时针或逆时针旋转 **一个位置** ，计为1步。旋转的最终目的是将字符串 `ring` 的一个字符与 `12:00` 方向对齐，并且这个字符必须等于字符 `key[i]` 。
2. 如果字符 `key[i]` 已经对齐到12:00方向，您需要按下中心按钮进行拼写，这也将算作 1 步。按完之后，您可以开始拼写 **key** 的下一个字符（下一阶段）, 直至完成所有拼写。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/10/22/ring.jpg)
<pre>
<strong>输入:</strong> https://assets.leetcode.com/uploads/2018/10/22/ring.jpg
<strong>输出:</strong> 4
<strong>解释:</strong>
对于 key 的第一个字符 'g'，已经在正确的位置, 我们只需要1步来拼写这个字符。
对于 key 的第二个字符 'd'，我们需要逆时针旋转 ring "godding" 2步使它变成 "ddinggo"。
当然, 我们还需要1步进行拼写。
因此最终的输出是 4。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ring = "godding", key = "godding"
<strong>输出:</strong> 13
</pre>

#### 提示:
* `1 <= ring.length, key.length <= 100`
* `ring` 和 `key` 只包含小写英文字母
* **保证** 字符串 `key` 一定可以由字符串  `ring` 旋转拼出

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let ring = ring.as_bytes();
        let key = key.as_bytes();
        let mut dp = vec![None; n];
        dp[0] = Some(0);

        for i in 0..key.len() {
            let mut tmp: Vec<Option<i32>> = vec![None; n];

            for j in 0..n {
                if ring[j] != key[i] {
                    continue;
                }

                for k in 0..n {
                    if let Some(x) = dp[k] {
                        let y = tmp[j].unwrap_or(i32::MAX);
                        let a = j.min(k) as i32;
                        let b = j.max(k) as i32;
                        tmp[j] = Some(y.min(x + b - a + 1).min(x + n as i32 - b + a + 1));
                    }
                }
            }

            dp = tmp;
        }

        dp.iter().map(|&x| x.unwrap_or(i32::MAX)).min().unwrap()
    }
}
```
