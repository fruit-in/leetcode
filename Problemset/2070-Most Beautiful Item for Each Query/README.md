# 2070. Most Beautiful Item for Each Query
You are given a 2D integer array `items` where <code>items[i] = [price<sub>i</sub>, beauty<sub>i</sub>]</code> denotes the price and beauty of an item respectively.

You are also given a **0-indexed** integer array `queries`. For each `queries[j]`, you want to determine the **maximum beauty** of an item whose **price** is **less than or equal** to `queries[j]`. If no such item exists, then the answer to this query is `0`.

Return *an array* `answer` *of the same length as* `queries` *where* `answer[j]` *is the answer to the* <code>j<sup>th</sup></code> *query*.

#### Example 1:
<pre>
<strong>Input:</strong> items = [[1,2],[3,2],[2,4],[5,6],[3,5]], queries = [1,2,3,4,5,6]
<strong>Output:</strong> [2,4,5,5,6,6]
<strong>Explanation:</strong>
- For queries[0]=1, [1,2] is the only item which has price <= 1. Hence, the answer for this query is 2.
- For queries[1]=2, the items which can be considered are [1,2] and [2,4].
  The maximum beauty among them is 4.
- For queries[2]=3 and queries[3]=4, the items which can be considered are [1,2], [3,2], [2,4], and [3,5].
  The maximum beauty among them is 5.
- For queries[4]=5 and queries[5]=6, all items can be considered.
  Hence, the answer for them is the maximum beauty of all items, i.e., 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> items = [[1,2],[1,2],[1,3],[1,4]], queries = [1]
<strong>Output:</strong> [4]
<strong>Explanation:</strong>
The price of every item is equal to 1, so we choose the item with the maximum beauty 4.
Note that multiple items can have the same price and/or beauty.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> items = [[10,1000]], queries = [5]
<strong>Output:</strong> [0]
<strong>Explanation:</strong>
No item has a price less than or equal to 5, so no item can be chosen.
Hence, the answer to the query is 0.
</pre>

#### Constraints:
* <code>1 <= items.length, queries.length <= 10<sup>5</sup></code>
* `items[i].length == 2`
* <code>1 <= price<sub>i</sub>, beauty<sub>i</sub>, queries[j] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        let mut answer = vec![0; queries.len()];

        items.sort_unstable_by_key(|item| (item[0], -item[1]));

        for i in 1..items.len() {
            items[i][1] = items[i][1].max(items[i - 1][1]);
        }

        for i in 0..queries.len() {
            let j = items
                .binary_search(&vec![queries[i], i32::MAX])
                .unwrap_err();

            answer[i] = match j {
                0 => 0,
                _ => items[j - 1][1],
            };
        }

        answer
    }
}
```
