# 1773. 统计匹配检索规则的物品数量
给你一个数组 `items` ，其中 <code>items[i] = [type<sub>i</sub>, color<sub>i</sub>, name<sub>i</sub>]</code> ，描述第 `i` 件物品的类型、颜色以及名称。

另给你一条由两个字符串 `ruleKey` 和 `ruleValue` 表示的检索规则。

如果第 `i` 件物品能满足下述条件之一，则认为该物品与给定的检索规则 **匹配** ：
* `ruleKey == "type"` 且 <code>ruleValue == type<sub>i</sub></code> 。
* `ruleKey == "color"` 且 <code>ruleValue == color<sub>i</sub></code> 。
* `ruleKey == "name"` 且 <code>ruleValue == name<sub>i</sub></code> 。

统计并返回 **匹配检索规则的物品数量** 。

#### 示例 1:
<pre>
<strong>输入:</strong> items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
<strong>输出:</strong> 1
<strong>解释:</strong> 只有一件物品匹配检索规则，这件物品是 ["computer","silver","lenovo"] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
<strong>输出:</strong> 2
<strong>解释:</strong> 只有两件物品匹配检索规则，这两件物品分别是 ["phone","blue","pixel"] 和 ["phone","gold","iphone"] 。注意，["computer","silver","phone"] 未匹配检索规则。
</pre>

#### 提示:
* <code>1 <= items.length <= 10<sup>4</sup></code>
* <code>1 <= type<sub>i</sub>.length, color<sub>i</sub>.length, name<sub>i</sub>.length, ruleValue.length <= 10</code>
* `ruleKey` 等于 `"type"`、`"color"` 或 `"name"`
* 所有字符串仅由小写字母组成

## 题解 (Rust)

### 1. 题解
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
