# 2260. 必须拿起的最小连续卡牌数
给你一个整数数组 `cards` ，其中 `cards[i]` 表示第 `i` 张卡牌的 **值** 。如果两张卡牌的值相同，则认为这一对卡牌 **匹配** 。

返回你必须拿起的最小连续卡牌数，以使在拿起的卡牌中有一对匹配的卡牌。如果无法得到一对匹配的卡牌，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> cards = [3,4,2,3,4,7]
<strong>输出:</strong> 4
<strong>解释:</strong> 拿起卡牌 [3,4,2,3] 将会包含一对值为 3 的匹配卡牌。注意，拿起 [4,2,3,4] 也是最优方案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cards = [1,0,5,3]
<strong>输出:</strong> -1
<strong>解释:</strong> 无法找出含一对匹配卡牌的一组连续卡牌。
</pre>

#### 提示:
* <code>1 <= cards.length <= 10<sup>5</sup></code>
* <code>0 <= cards[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
        let mut indices = HashMap::new();
        let mut ret = usize::MAX;

        for i in 0..cards.len() {
            if let Some(j) = indices.insert(cards[i], i) {
                ret = ret.min(i - j + 1);
            }
        }

        if ret == usize::MAX {
            return -1;
        }

        ret as i32
    }
}
```
