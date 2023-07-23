# 1268. Search Suggestions System
Given an array of strings `products` and a string `searchWord`. We want to design a system that suggests at most three product names from `products` after each character of `searchWord` is typed. Suggested products should have common prefix with the searchWord. If there are more than three products with a common prefix return the three lexicographically minimums products.

Return *list of lists* of the suggested `products` after each character of `searchWord` is typed.

#### Example 1:
<pre>
<strong>Input:</strong> products = ["mobile","mouse","moneypot","monitor","mousepad"], searchWord = "mouse"
<strong>Output:</strong> [
["mobile","moneypot","monitor"],
["mobile","moneypot","monitor"],
["mouse","mousepad"],
["mouse","mousepad"],
["mouse","mousepad"]
]
<strong>Explanation:</strong> products sorted lexicographically = ["mobile","moneypot","monitor","mouse","mousepad"]
After typing m and mo all products match and we show user ["mobile","moneypot","monitor"]
After typing mou, mous and mouse the system suggests ["mouse","mousepad"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> products = ["havana"], searchWord = "havana"
<strong>Output:</strong> [["havana"],["havana"],["havana"],["havana"],["havana"],["havana"]]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> products = ["bags","baggage","banner","box","cloths"], searchWord = "bags"
<strong>Output:</strong> [["baggage","bags","banner"],["baggage","bags","banner"],["baggage","bags"],["bags"]]
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> products = ["havana"], searchWord = "tatiana"
<strong>Output:</strong> [[],[],[],[],[],[],[]]
</pre>

#### Constraints:
* `1 <= products.length <= 1000`
* There are no repeated elements in `products`.
* `1 <= Σ products[i].length <= 2 * 10^4`
* All characters of `products[i]` are lower-case English letters.
* `1 <= searchWord.length <= 1000`
* All characters of `searchWord` are lower-case English letters.

## Solutions (Rust)

### 1. Solution
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
