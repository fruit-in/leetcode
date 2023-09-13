# 2070. 每一个查询的最大美丽值
给你一个二维整数数组 `items` ，其中 <code>items[i] = [price<sub>i</sub>, beauty<sub>i</sub>]</code> 分别表示每一个物品的 **价格** 和 **美丽值** 。

同时给你一个下标从 **0** 开始的整数数组 `queries` 。对于每个查询 `queries[j]` ，你想求出价格小于等于 `queries[j]` 的物品中，**最大的美丽值** 是多少。如果不存在符合条件的物品，那么查询的结果为 `0` 。

请你返回一个长度与 `queries` 相同的数组 `answer`，其中 `answer[j]`是第 `j` 个查询的答案。

#### 示例 1:
<pre>
<strong>输入:</strong> items = [[1,2],[3,2],[2,4],[5,6],[3,5]], queries = [1,2,3,4,5,6]
<strong>输出:</strong> [2,4,5,5,6,6]
<strong>解释:</strong>
- queries[0]=1 ，[1,2] 是唯一价格 <= 1 的物品。所以这个查询的答案为 2 。
- queries[1]=2 ，符合条件的物品有 [1,2] 和 [2,4] 。
  它们中的最大美丽值为 4 。
- queries[2]=3 和 queries[3]=4 ，符合条件的物品都为 [1,2] ，[3,2] ，[2,4] 和 [3,5] 。
  它们中的最大美丽值为 5 。
- queries[4]=5 和 queries[5]=6 ，所有物品都符合条件。
  所以，答案为所有物品中的最大美丽值，为 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> items = [[1,2],[1,2],[1,3],[1,4]], queries = [1]
<strong>输出:</strong> [4]
<strong>解释:</strong>
每个物品的价格均为 1 ，所以我们选择最大美丽值 4 。
注意，多个物品可能有相同的价格和美丽值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> items = [[10,1000]], queries = [5]
<strong>输出:</strong> [0]
<strong>解释:</strong>
没有物品的价格小于等于 5 ，所以没有物品可以选择。
因此，查询的结果为 0 。
</pre>

#### 提示:
* <code>1 <= items.length, queries.length <= 10<sup>5</sup></code>
* `items[i].length == 2`
* <code>1 <= price<sub>i</sub>, beauty<sub>i</sub>, queries[j] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
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
