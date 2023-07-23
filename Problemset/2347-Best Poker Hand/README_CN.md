# 2347. 最好的扑克手牌
给你一个整数数组 `ranks` 和一个字符数组 `suit` 。你有 `5` 张扑克牌，第 `i` 张牌大小为 `ranks[i]` ，花色为 `suits[i]` 。

下述是从好到坏你可能持有的 **手牌类型** ：
1. `"Flush"`：同花，五张相同花色的扑克牌。
2. `"Three of a Kind"`：三条，有 3 张大小相同的扑克牌。
3. `"Pair"`：对子，两张大小一样的扑克牌。
4. `"High Card"`：高牌，五张大小互不相同的扑克牌。

请你返回一个字符串，表示给定的 5 张牌中，你能组成的 **最好手牌类型** 。

**注意：**返回的字符串 **大小写** 需与题目描述相同。

#### 示例 1:
<pre>
<strong>输入:</strong> ranks = [13,2,3,1,9], suits = ["a","a","a","a","a"]
<strong>输出:</strong> "Flush"
<strong>解释:</strong> 5 张扑克牌的花色相同，所以返回 "Flush" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ranks = [4,4,2,4,4], suits = ["d","a","a","b","c"]
<strong>输出:</strong> "Three of a Kind"
<strong>解释:</strong> 第一、二和四张牌组成三张相同大小的扑克牌，所以得到 "Three of a Kind" 。
注意我们也可以得到 "Pair" ，但是 "Three of a Kind" 是更好的手牌类型。
有其他的 3 张牌也可以组成 "Three of a Kind" 手牌类型。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> ranks = [10,10,2,12,9], suits = ["a","b","c","a","d"]
<strong>输出:</strong> "Pair"
<strong>解释:</strong> 第一和第二张牌大小相同，所以得到 "Pair" 。
我们无法得到 "Flush" 或者 "Three of a Kind" 。
</pre>

#### 提示:
* `ranks.length == suits.length == 5`
* `1 <= ranks[i] <= 13`
* `'a' <= suits[i] <= 'd'`
* 任意两张扑克牌不会同时有相同的大小和花色。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def bestHand(self, ranks: List[int], suits: List[str]) -> str:
        ranks_counter = Counter(ranks)
        suits_counter = Counter(suits)

        if len(suits_counter) == 1:
            return "Flush"
        elif max(ranks_counter.values()) > 2:
            return "Three of a Kind"
        elif max(ranks_counter.values()) > 1:
            return "Pair"
        else:
            return "High Card"
```
