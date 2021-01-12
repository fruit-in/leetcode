# 1268. 搜索推荐系统
给你一个产品数组 `products` 和一个字符串 `searchWord` ，`products`  数组中每个产品都是一个字符串。

请你设计一个推荐系统，在依次输入单词 `searchWord` 的每一个字母后，推荐 `products` 数组中前缀与 `searchWord` 相同的最多三个产品。如果前缀相同的可推荐产品超过三个，请按字典序返回最小的三个。

请你以二维列表的形式，返回在输入 `searchWord` 每个字母后相应的推荐产品的列表。

#### 示例 1:
<pre>
<strong>输入:</strong> products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
<strong>输出:</strong> [
["mobile","moneypot","monitor"],
["mobile","moneypot","monitor"],
["mouse","mousepad"],
["mouse","mousepad"],
["mouse","mousepad"]
]
<strong>解释:</strong> 按字典序排序后的产品列表是 ["mobile","moneypot","monitor","mouse","mousepad"]
输入 m 和 mo，由于所有产品的前缀都相同，所以系统返回字典序最小的三个产品 ["mobile","moneypot","monitor"]
输入 mou， mous 和 mouse 后系统都返回 ["mouse","mousepad"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> products = ["havana"], searchWord = "havana"
<strong>输出:</strong> [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> products = ["bags","baggage","banner","box","cloths"], searchWord = "bags"
<strong>输出:</strong> [["baggage","bags","banner"],["baggage","bags","banner"],["baggage","bags"],["bags"]]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> products = ["havana"], searchWord = "tatiana"
<strong>输出:</strong> [[],[],[],[],[],[],[]]
</pre>

#### 提示:
* `1 <= products.length <= 1000`
* `1 <= Σ products[i].length <= 2 * 10^4`
* `products[i]` 中所有的字符都是小写英文字母。
* `1 <= searchWord.length <= 1000`
* `searchWord` 中所有字符都是小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut ret = vec![];

        products.sort_unstable();

        for i in 1..=search_word.len() {
            products.retain(|s| s.starts_with(search_word.get(0..i).unwrap()));
            ret.push(products.iter().take(3).cloned().collect());
        }

        ret
    }
}
```
