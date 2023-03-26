# 2363. Merge Similar Items
You are given two 2D integer arrays, `items1` and `items2`, representing two sets of `items`. Each array items has the following properties:

* <code>items[i] = [value<sub>i</sub>, weight<sub>i</sub>]</code> where <code>value<sub>i</sub></code> represents the **value** and <code>weight<sub>i</sub></code> represents the **weight** of the <code>i<sup>th</sup></code> item.
* The value of each item in `items` is **unique**.

Return *a 2D integer array* `ret` *where* <code>ret[i] = [value<sub>i</sub>, weight<sub>i</sub>]</code>, *with* <code>weight<sub>i</sub></code> *being the **sum of weights** of all items with value* <code>value<sub>i</sub></code>.

**Note:** `ret` should be returned in **ascending** order by value.

#### Example 1:
<pre>
<strong>Input:</strong> items1 = [[1,1],[4,5],[3,8]], items2 = [[3,1],[1,5]]
<strong>Output:</strong> [[1,6],[3,9],[4,5]]
<strong>Explanation:</strong>
The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 5, total weight = 1 + 5 = 6.
The item with value = 3 occurs in items1 with weight = 8 and in items2 with weight = 1, total weight = 8 + 1 = 9.
The item with value = 4 occurs in items1 with weight = 5, total weight = 5.
Therefore, we return [[1,6],[3,9],[4,5]].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> items1 = [[1,1],[3,2],[2,3]], items2 = [[2,1],[3,2],[1,3]]
<strong>Output:</strong> [[1,4],[2,4],[3,4]]
<strong>Explanation:</strong>
The item with value = 1 occurs in items1 with weight = 1 and in items2 with weight = 3, total weight = 1 + 3 = 4.
The item with value = 2 occurs in items1 with weight = 3 and in items2 with weight = 1, total weight = 3 + 1 = 4.
The item with value = 3 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4.
Therefore, we return [[1,4],[2,4],[3,4]].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> items1 = [[1,3],[2,2]], items2 = [[7,1],[2,2],[1,4]]
<strong>Output:</strong> [[1,7],[2,4],[7,1]]
<strong>Explanation:</strong>
The item with value = 1 occurs in items1 with weight = 3 and in items2 with weight = 4, total weight = 3 + 4 = 7.
The item with value = 2 occurs in items1 with weight = 2 and in items2 with weight = 2, total weight = 2 + 2 = 4.
The item with value = 7 occurs in items2 with weight = 1, total weight = 1.
Therefore, we return [[1,7],[2,4],[7,1]].
</pre>

#### Constraints:
* `1 <= items1.length, items2.length <= 1000`
* `items1[i].length == items2[i].length == 2`
* <code>1 <= value<sub>i</sub>, weight<sub>i</sub> <= 1000</code>
* Each <code>value<sub>i</sub></code> in `items1` is **unique**.
* Each <code>value<sub>i</sub></code> in `items2` is **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items = HashMap::new();

        for item in items1.iter().chain(items2.iter()) {
            items
                .entry(item[0])
                .and_modify(|w| *w += item[1])
                .or_insert(item[1]);
        }

        let mut ret = items
            .into_iter()
            .map(|(v, w)| vec![v, w])
            .collect::<Vec<_>>();
        ret.sort_unstable();

        ret
    }
}
```
