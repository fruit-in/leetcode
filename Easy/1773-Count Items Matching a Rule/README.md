# 1773. Count Items Matching a Rule
You are given an array `items`, where each <code>items[i] = [type<sub>i</sub>, color<sub>i</sub>, name<sub>i</sub>]</code> describes the type, color, and name of the <code>i<sup>th</sup></code> item. You are also given a rule represented by two strings, `ruleKey` and `ruleValue`.

The <code>i<sup>th</sup></code> item is said to match the rule if **one** of the following is true:
* `ruleKey == "type"` and <code>ruleValue == type<sub>i</sub></code>.
* `ruleKey == "color"` and <code>ruleValue == color<sub>i</sub></code>.
* `ruleKey == "name"` and <code>ruleValue == name<sub>i</sub></code>.

Return *the number of items that match the given rule*.

#### Example 1:
<pre>
<strong>Input:</strong> items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one item matching the given rule, which is ["computer","silver","lenovo"].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are only two items matching the given rule, which are ["phone","blue","pixel"] and ["phone","gold","iphone"]. Note that the item ["computer","silver","phone"] does not match.
</pre>

#### Constraints:
* <code>1 <= items.length <= 10<sup>4</sup></code>
* <code>1 <= type<sub>i</sub>.length, color<sub>i</sub>.length, name<sub>i</sub>.length, ruleValue.length <= 10</code>
* `ruleKey` is equal to either `"type"`, `"color"`, or `"name"`.
* All strings consist only of lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_key = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            _ => 2,
        };

        items
            .iter()
            .filter(|item| item[rule_key] == rule_value)
            .count() as i32
    }
}
```
